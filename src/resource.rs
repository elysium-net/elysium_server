use crate::database::Database;
use crate::error::Error;
use crate::{chat, config, user};
use elysium_rust::chat::v1::ChannelPermission;
use elysium_rust::common::v1::ErrorCode;
use elysium_rust::{ResourceId, ResourceMeta};
use std::path::{Path, PathBuf};
use surrealdb::types::SurrealValue;
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio_util::io::ReaderStream;
use tonic::codegen::tokio_stream::{Stream, StreamExt};

/// Chunk Size of a byte data stream.
pub const CHUNK_SIZE: usize = 1024 * 2;

/// Built-in namespace.
pub const BUILTIN_NAMESPACE: &str = "elysium";

/// Built-in user icon key.
pub const DEFAULT_ICON_KEY: &str = "default_icon.png";

pub async fn create(
    database: &Database,
    desc: ResourceDescriptor,
) -> Result<ResourceDescriptor, Error> {
    if exists(database, &desc.id).await? {
        return Err(Error::new(
            ErrorCode::AlreadyExists,
            "Resource already exists",
        ));
    }

    let desc: Option<ResourceDescriptor> = database
        .insert(("resource", construct_id(&desc.id)))
        .content(desc)
        .await?;

    Ok(desc.ok_or(Error::new(ErrorCode::Internal, "Failed to create resource"))?)
}

pub async fn get(
    database: &Database,
    resource_id: &ResourceId,
) -> Result<Option<ResourceDescriptor>, Error> {
    let resource: Option<ResourceDescriptor> = database
        .select(("resource", construct_id(resource_id)))
        .await?;

    Ok(resource)
}

pub async fn exists(database: &Database, resource_id: &ResourceId) -> Result<bool, Error> {
    let resource = get(database, resource_id).await?;
    Ok(resource.is_some())
}

pub async fn is_download_authorized(
    database: &Database,
    desc: &ResourceDescriptor,
    user: &str,
) -> Result<bool, Error> {
    let mut authorized = false;
    let user = user::get(database, user)
        .await?
        .ok_or(Error::new(ErrorCode::NotFound, "User not found"))?;

    if from_builtin(&desc.id).is_some() {
        authorized = true;
    } else if let Some(channel) = chat::get_channel(database, &desc.id.namespace).await?
        && channel.members.contains_key(&user.user_id)
    {
        authorized = true;
    }

    Ok(authorized)
}

pub async fn is_upload_authorized(
    database: &Database,
    desc: &ResourceDescriptor,
    user: &str,
) -> Result<bool, Error> {
    let mut authorized = false;
    let user = user::get(database, user)
        .await?
        .ok_or(Error::new(ErrorCode::NotFound, "User not found"))?;

    if let Some(channel) = chat::get_channel(database, &desc.id.namespace).await? {
        let perm =
            chat::get_channel_member_perm(database, &channel.channel_id, &user.user_id).await?;

        authorized = perm == ChannelPermission::Manager || perm == ChannelPermission::ReadWrite;
    }

    Ok(authorized)
}

pub async fn read(id: ResourceId) -> Result<impl Stream<Item = Result<Vec<u8>, Error>>, Error> {
    let path = build_path(&id);

    let file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .append(false)
        .open(path)
        .await
        .map_err(|e| {
            tracing::error!("Failed reading file: {e}");
            Error::new(ErrorCode::Internal, "Failed to read resource")
        })?;

    let stream = ReaderStream::with_capacity(file, CHUNK_SIZE).map(|res| {
        res.map_err(|err| {
            tracing::error!("Failed reading file: {err}");
            Error::new(ErrorCode::Internal, "Failed to read resource")
        })
        .map(|by| by.to_vec())
    });

    Ok(stream)
}

pub async fn write(
    id: ResourceId,
    stream: impl Stream<Item = Result<Vec<u8>, Error>>,
) -> Result<(), Error> {
    let path = build_path(&id);

    let file = fs::File::create(path).await.map_err(|e| {
        tracing::error!("Failed opening file: {e}");
        Error::new(ErrorCode::Internal, "Failed to write resource")
    })?;

    tokio::pin!(stream);

    let mut buf = BufWriter::with_capacity(CHUNK_SIZE, file);

    while let Some(data) = stream.next().await {
        buf.write_all(&data?).await.map_err(|e| {
            tracing::error!("Failed writing file: {e}");
            Error::new(ErrorCode::Internal, "Failed to write resource")
        })?;
    }

    buf.flush().await.map_err(|e| {
        tracing::error!("Failed flushing file: {e}");
        Error::new(ErrorCode::Internal, "Failed to write resource")
    })?;

    Ok(())
}

pub fn from_builtin(id: &ResourceId) -> Option<PathBuf> {
    if id.namespace.as_str() != BUILTIN_NAMESPACE {
        return None;
    }

    match id.key.as_str() {
        DEFAULT_ICON_KEY => Some(PathBuf::from("elysium/default_icon.png")),
        _ => None,
    }
}

fn construct_id(id: &ResourceId) -> String {
    format!("{}:{}", id.namespace, id.key)
}

fn build_path(id: &ResourceId) -> PathBuf {
    Path::new(&config::get().service_resource_dir)
        .join(&id.namespace)
        .join(&id.key)
}

#[derive(Clone, Debug, SurrealValue)]
pub struct ResourceDescriptor {
    pub id: ResourceId,
    pub meta: ResourceMeta,
    pub user_id: String,
}
