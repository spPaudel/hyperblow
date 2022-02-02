use crate::ui::files::FilesState;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::torrent_parser;

#[derive(Debug)]
enum FileType {
    REGULAR,
    DIRECTORY,
}

#[derive(Debug)]
struct File {
    name: String,
    file_type: FileType,
    inner_files: Option<Vec<File>>,
}

impl File {
    fn contains(&self, fileName: &String) -> (Option<usize>, bool) {
        let mut index = None;
        let mut doesExist = false;
        if let Some(files) = &self.inner_files {
            for (i, x) in files.iter().enumerate() {
                if x.name == *fileName {
                    index = Some(i);
                    doesExist = true;
                }
            }
        }
        (index, doesExist)
    }

    fn add_file(&mut self, file: File) -> usize {
        let mut index = 0;
        if let Some(i) = &mut self.inner_files {
            i.push(file);
            index = i.len() - 1
        }
        index
    }
}

// Starting Point for the working thread
pub fn start(fileState: Arc<Mutex<FilesState>>, torrent_file_path: &String) {
    // Get the argument at index 1 from the CLI command "rtourent xyz.torrent"
    // So that we can get the name of the file i.e xyz.torrentj
    let (torrentParsed, info_hashBytes) = torrent_parser::parse_file(&torrent_file_path);

    let mut root_file = File {
        name: String::from("yo"),
        file_type: FileType::DIRECTORY,
        inner_files: Some(Vec::new()),
    };

    let mut afile = &mut root_file;

    if let Some(files) = &torrentParsed.info.files {
        for file in files {
            for x in 0..file.path.len() {
                if !afile.contains(&file.path[x]).1 {
                    let last_path_index = file.path.len() - 1;
                    let index = afile.add_file(File {
                        name: String::from(&file.path[x]),
                        file_type: if x == last_path_index {
                            FileType::REGULAR
                        } else {
                            FileType::DIRECTORY
                        },
                        inner_files: if x == last_path_index {
                            None
                        } else {
                            Some(vec![])
                        },
                    });
                    if let Some(files) = &mut afile.inner_files {
                        let afile = &mut (*files)[index];
                    }
                }
            }
            afile = &mut root_file
        }
    }

    println!("{:?}", root_file);

    //   let percentEncodedInfoHash = percent_encoder::encode(info_hashBytes);
    //  println!("{:?}", percentEncodedInfoHash);
    //
    //    let client = Client::new();
    //    let uri = format!(
    //        "{}?info_hash={}&peer_id=RISHADBANIYA_1234567&port=6881",
    //        &torrentParsed.announce, &percentEncodedInfoHash
    //    )
    //    .parse()?;
    //    println!("{}", uri);
    //
    //    let resp = client.get(uri).await?;
    //    let body: Vec<u8> = (hyper::body::to_bytes(resp.into_body()).await?)
    //        .into_iter()
    //        .collect();
    //
    //    let tracker_response: TrackerResponse = serde_bencode::de::from_bytes(&body)?;
    //    println!("{}", String::from_utf8_lossy(&body));
    //    println!("{:?}", tracker_response);
    //
    //    let mut allTrackers: Vec<String> = vec![torrentParsed.announce.clone()];
    //
    //    if let Some(announce_list) = torrentParsed.announce_list {
    //        for tracker in announce_list {
    //            allTrackers.push(tracker[0].clone());
    //        }
    //    }
    //
    //    println!("All trackers are {:?}", allTrackers);
    //
    //    Ok(())
}

//#[derive(Debug, Deserialize)]
//struct TrackerResponse {
//    #[serde(rename = "failure reason")]
//    failure_reason: Option<String>,
//    #[serde(rename = "warning message")]
//    warning_message: Option<String>,
//    interval: Option<i64>,
//    #[serde(rename = "min interval")]
//   min_interval: Option<i64>,
//    #[serde(rename = "tracker id")]
//    tracker_id: Option<String>,
//    complete: Option<i64>,
//    incomplete: Option<i64>,
//    peers: Vec<Peers>,
//}
//
//#[derive(Debug, Deserialize)]
//struct Peers {
//    #[serde(rename = "peer id")]
//    peer_id: String,
//    ip: String,
//    port: String,
//}
