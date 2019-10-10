#![warn(rust_2018_idioms)]

use tokio::fs::File;
use tokio::prelude::*;

use std::io::prelude::*;
use tempfile::NamedTempFile;

/*
use rand::{distributions, thread_rng, Rng};
use std::fs;
use std::io::SeekFrom;
use tempfile::Builder as TmpBuilder;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_fs::*;
*/

const HELLO: &[u8] = b"hello world...";

#[tokio::test]
async fn basic_read() {
    let mut tempfile = tempfile();
    tempfile.write_all(HELLO).unwrap();

    let mut file = File::open(tempfile.path()).await.unwrap();

    let mut buf = [0; 1024];
    let n = file.read(&mut buf).await.unwrap();

    assert_eq!(n, HELLO.len());
    assert_eq!(&buf[..n], HELLO);
}

#[tokio::test]
async fn basic_write() {
    let tempfile = tempfile();

    let mut file = File::create(tempfile.path()).await.unwrap();

    file.write_all(HELLO).await.unwrap();
    file.flush().await.unwrap();

    let file = std::fs::read(tempfile.path()).unwrap();
    assert_eq!(file, HELLO);
}

fn tempfile() -> NamedTempFile {
    NamedTempFile::new().unwrap()
}

#[cfg(unix)]
mod unix_text {
    use super::*;
    use std::os::unix::io::AsRawFd;

    #[test]
    fn unix_can_access_file_fd() {
        let tempfile = tempfile();
        let file = std::fs::File::create(tempfile.path()).unwrap();
        let file_fd = file.as_raw_fd();
        let async_file = File::from_std(file);
        let async_file_fd = async_file.as_raw_fd();
        assert_eq!(file_fd, async_file_fd)
    }
}

/*
mod pool;

#[test]
fn read_write() {
    const NUM_CHARS: usize = 16 * 1_024;

    let dir = TmpBuilder::new()
        .prefix("tokio-fs-tests")
        .tempdir()
        .unwrap();
    let file_path = dir.path().join("read_write.txt");

    let contents: Vec<u8> = thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(NUM_CHARS)
        .collect::<String>()
        .into();

    let file_path_2 = file_path.clone();
    let contents_2 = contents.clone();

    pool::run(async move {
        let mut file = File::create(file_path).await?;
        let metadata = file.metadata().await?;
        assert!(metadata.is_file());
        file.write(&contents).await?;
        file.sync_all().await?;
        Ok(())
    });

    let dst = fs::read(&file_path_2).unwrap();
    assert_eq!(dst, contents_2);

    pool::run(async move {
        let buf = read(file_path_2).await?;
        assert_eq!(buf, contents_2);
        Ok(())
    });
}

#[test]
fn read_write_helpers() {
    const NUM_CHARS: usize = 16 * 1_024;

    let dir = TmpBuilder::new()
        .prefix("tokio-fs-tests")
        .tempdir()
        .unwrap();
    let file_path = dir.path().join("read_write_all.txt");

    let contents: Vec<u8> = thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(NUM_CHARS)
        .collect::<String>()
        .into();

    let file_path_2 = file_path.clone();
    let contents_2 = contents.clone();

    pool::run(async move {
        write(file_path, contents).await?;
        Ok(())
    });

    let dst = fs::read(&file_path_2).unwrap();
    assert_eq!(dst, contents_2);

    pool::run(async move {
        let buf = read(file_path_2).await?;
        assert_eq!(buf, contents_2);
        Ok(())
    });
}

#[test]
fn metadata() {
    let dir = TmpBuilder::new()
        .prefix("tokio-fs-tests")
        .tempdir()
        .unwrap();
    let file_path = dir.path().join("metadata.txt");

    pool::run(async move {
        assert!(tokio_fs::metadata(file_path.clone()).await.is_err());
        File::create(file_path.clone()).await?;
        let metadata = tokio_fs::metadata(file_path.clone()).await?;
        assert!(metadata.is_file());
        Ok(())
    });
}

#[test]
fn seek() {
    let dir = TmpBuilder::new()
        .prefix("tokio-fs-tests")
        .tempdir()
        .unwrap();
    let file_path = dir.path().join("seek.txt");

    pool::run(async move {
        let mut options = OpenOptions::new();

        options.create(true).read(true).write(true);

        let mut file = options.open(file_path).await.unwrap();

        assert!(file.write(b"Hello, world!").await.is_ok());
        file.seek(SeekFrom::End(-6)).await.unwrap();
        let mut buf = vec![0; 5];
        assert!(file.read(buf.as_mut()).await.is_ok());
        assert_eq!(buf, b"world");
        file.seek(SeekFrom::Start(0)).await.unwrap();
        let mut buf = vec![0; 5];
        assert!(file.read(buf.as_mut()).await.is_ok());
        assert_eq!(buf, b"Hello");
        Ok(())
    });
}

#[test]
fn clone() {
    use std::io::prelude::*;

    let dir = TmpBuilder::new()
        .prefix("tokio-fs-tests")
        .tempdir()
        .unwrap();
    let file_path = dir.path().join("clone.txt");
    let file_path_2 = file_path.clone();

    pool::run(async move {
        let mut file = File::create(file_path.clone()).await.unwrap();
        let mut clone = file.try_clone().await.unwrap();
        assert!(AsyncWriteExt::write(&mut file, b"clone ").await.is_ok());
        assert!(AsyncWriteExt::write(&mut clone, b"successful")
            .await
            .is_ok());
        Ok(())
    });

    let mut file = std::fs::File::open(&file_path_2).unwrap();

    let mut dst = vec![];
    file.read_to_end(&mut dst).unwrap();

    assert_eq!(dst, b"clone successful")
}
*/
