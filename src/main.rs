use clap::{Arg, Command, value_parser};
use decompress::ExtractOptsBuilder;
use oxrdfxml::RdfXmlParser;
use oxrdfxml::RdfXmlSerializer;
use oxttl::TurtleParser;
use oxttl::TurtleSerializer;
use std::env;
use std::fs;
use std::path::PathBuf;

const RDF_XML_TO_TURTLE_NAME: &str = "rdf-xml-to-turtle";
const RDF_TURTLE_TO_RDF_XML_NAME: &str = "rdf-turtle-to-rdf-xml";
const UNZIP_FILES_TO_SINGLE_DIR_NAME: &str = "unzip-files-to-single-dir";
const MOVE_FILES_TO_THEIR_DIRS_NAME: &str = "move-files-to-their-dirs";
const CONCEPTUAL_MAPPER_NAME: &str = "conceptual-mapper";

fn main() {
    let rdf_xml_to_turtle_cmd = Command::new(RDF_XML_TO_TURTLE_NAME)
        .about(RDF_XML_TO_TURTLE_NAME)
        .arg(
            Arg::new("input_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("output_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(2)
                .required(true),
        );

    let rdf_turtle_to_rdf_xml_cmd = Command::new(RDF_TURTLE_TO_RDF_XML_NAME)
        .about(RDF_TURTLE_TO_RDF_XML_NAME)
        .arg(
            Arg::new("input_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("output_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(2)
                .required(true),
        );

    let unzip_file_to_single_dir_cmd = Command::new(UNZIP_FILES_TO_SINGLE_DIR_NAME)
        .about(UNZIP_FILES_TO_SINGLE_DIR_NAME)
        .arg(
            Arg::new("archive_file_path")
                .value_parser(value_parser!(PathBuf))
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("output_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(2)
                .required(true),
        )
        .arg(Arg::new("excludes").index(3).num_args(0..));

    let move_files_to_their_dirs_cmd = Command::new(MOVE_FILES_TO_THEIR_DIRS_NAME)
        .about(MOVE_FILES_TO_THEIR_DIRS_NAME)
        .arg(
            Arg::new("input_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("output_dir_path")
                .value_parser(value_parser!(PathBuf))
                .index(2)
                .required(true),
        );
    let conceptual_mapper_cmd = Command::new(CONCEPTUAL_MAPPER_NAME)
        .about(CONCEPTUAL_MAPPER_NAME)
        .arg(
            Arg::new("config_file_path")
                .value_parser(value_parser!(PathBuf))
                .index(1)
                .required(true),
        );

    let cli_matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(rdf_xml_to_turtle_cmd)
        .subcommand(rdf_turtle_to_rdf_xml_cmd)
        .subcommand(unzip_file_to_single_dir_cmd)
        .subcommand(move_files_to_their_dirs_cmd)
        .subcommand(conceptual_mapper_cmd)
        .get_matches();

    let (subcommand_name, subcommand_arguments) =
        cli_matches.subcommand().expect("cannot parse the args");

    match subcommand_name {
        RDF_XML_TO_TURTLE_NAME => {
            let input_dir_path: &PathBuf = subcommand_arguments
                .get_one("input_dir_path")
                .expect("`input_dir_path` is required");
            let output_dir_path: &PathBuf = subcommand_arguments
                .get_one("output_dir_path")
                .expect("`output_dir_path` is required");

            rdf_xml_to_turtle(input_dir_path, output_dir_path);
        }
        RDF_TURTLE_TO_RDF_XML_NAME => {
            let input_dir_path: &PathBuf = subcommand_arguments
                .get_one("input_dir_path")
                .expect("`input_dir_path` is required");
            let output_dir_path: &PathBuf = subcommand_arguments
                .get_one("output_dir_path")
                .expect("`output_dir_path` is required");

            rdf_turtle_to_rdf_xml(input_dir_path, output_dir_path);
        }
        UNZIP_FILES_TO_SINGLE_DIR_NAME => {
            let archive_file_path: &PathBuf = subcommand_arguments
                .get_one("archive_file_path")
                .expect("`archive_file_path` is required");
            let output_dir_path: &PathBuf = subcommand_arguments
                .get_one("output_dir_path")
                .expect("`output_dir_path` is required");
            let excludes = subcommand_arguments
                .get_many::<String>("excludes")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<_>>();

            unzip_files_to_single_dir(archive_file_path, output_dir_path, excludes);
        }
        MOVE_FILES_TO_THEIR_DIRS_NAME => {
            let input_dir_path: &PathBuf = subcommand_arguments
                .get_one("input_dir_path")
                .expect("`input_dir_path` is required");
            let output_dir_path: &PathBuf = subcommand_arguments
                .get_one("output_dir_path")
                .expect("`output_dir_path` is required");

            move_files_to_their_dirs(input_dir_path, output_dir_path);
        }
        CONCEPTUAL_MAPPER_NAME => {
            let config_file_path: &PathBuf = subcommand_arguments
                .get_one("config_file_path")
                .expect("`config_file_path` is required");

            conceptual_mapper_lib::run_mappings(config_file_path.to_owned());
        }
        _ => (),
    }
}

fn rdf_xml_to_turtle(input_dir_path: &PathBuf, output_dir_path: &PathBuf) {
    fs::create_dir_all(&output_dir_path).unwrap();

    let input_file_paths = fs::read_dir(input_dir_path).unwrap();

    for input_file_path in input_file_paths {
        let input_file_path = input_file_path.unwrap().path();

        let file_basename = input_file_path.file_stem().unwrap().display();
        let rdf_xml_file_contents = std::fs::read_to_string(&input_file_path).unwrap();

        let mut serializer = TurtleSerializer::new().for_writer(Vec::new());

        for triple in RdfXmlParser::new().for_slice(rdf_xml_file_contents.as_bytes()) {
            let triple = triple.unwrap();

            serializer.serialize_triple(triple.as_ref()).unwrap();
        }

        let result = serializer.finish().unwrap();

        std::fs::write(
            output_dir_path.join(format!("{}.ttl", file_basename)),
            result,
        )
        .unwrap();
    }
}

fn rdf_turtle_to_rdf_xml(input_dir_path: &PathBuf, output_dir_path: &PathBuf) {
    fs::create_dir_all(&output_dir_path).unwrap();

    let input_file_paths = fs::read_dir(input_dir_path).unwrap();

    for input_file_path in input_file_paths {
        let input_file_path = input_file_path.unwrap().path();

        let file_basename = input_file_path.file_stem().unwrap().display();
        let rdf_xml_file_contents = std::fs::read_to_string(&input_file_path).unwrap();

        let mut serializer = RdfXmlSerializer::new().for_writer(Vec::new());

        for triple in TurtleParser::new().for_slice(rdf_xml_file_contents.as_bytes()) {
            let triple = triple.unwrap();

            serializer.serialize_triple(triple.as_ref()).unwrap();
        }

        let result = serializer.finish().unwrap();

        std::fs::write(
            output_dir_path.join(format!("{}.rdf", file_basename)),
            result,
        )
        .unwrap();
    }
}

fn unzip_files_to_single_dir(
    archive_file_path: &PathBuf,
    output_dir_path: &PathBuf,
    excludes: Vec<&str>,
) {
    fs::create_dir_all(&output_dir_path).unwrap();

    let xml_files_glob = glob::Pattern::new("**/*.xml").expect("wrong glob pattern for XML files");

    // decompress the zip archive
    let tmp_dir_path = output_dir_path.join("../tmp");

    fs::create_dir_all(&tmp_dir_path).unwrap();

    let extract_opts = ExtractOptsBuilder::default().strip(1).build().unwrap();

    decompress::decompress(archive_file_path, &tmp_dir_path, &extract_opts).unwrap();

    // remove not needed files
    for exclude in excludes {
        std::fs::remove_dir_all(&tmp_dir_path.join(exclude)).unwrap();
    }

    // copy files to a single dir
    let sub_directories = std::fs::read_dir(&tmp_dir_path).unwrap();

    for sub_directory in sub_directories {
        match sub_directory {
            Ok(entry) => {
                let current_path = entry.path();

                if current_path.is_dir() {
                    let current_dir_name = current_path.file_name().unwrap().display().to_string();

                    let xml_files_globbed_path = current_path.join(xml_files_glob.as_str());

                    for xml_file_path in glob::glob(xml_files_globbed_path.to_str().unwrap())
                        .expect("Failed to read glob pattern")
                        .filter_map(Result::ok)
                    {
                        let xml_file_name =
                            xml_file_path.file_name().unwrap().display().to_string();

                        std::fs::rename(
                            &xml_file_path,
                            &output_dir_path
                                .join(format!("{}__{}", &current_dir_name, &xml_file_name)),
                        )
                        .unwrap();
                    }
                }
            }
            Err(e) => {
                println!("  entry error: {:?}", e);
            }
        }
    }

    // remove the tmp dir
    std::fs::remove_dir_all(&tmp_dir_path).unwrap();
}

fn move_files_to_their_dirs(input_dir_path: &PathBuf, output_dir_path: &PathBuf) {
    fs::create_dir_all(&output_dir_path).unwrap();

    let rdf_xml_files = std::fs::read_dir(&input_dir_path).unwrap();

    for rdf_xml_file in rdf_xml_files {
        match rdf_xml_file {
            Ok(entry) => {
                let current_path = entry.path();

                if current_path.is_file() {
                    let current_rdf_xml_file_name =
                        current_path.file_name().unwrap().display().to_string();
                    let current_rdf_xml_file_name_splitted =
                        current_rdf_xml_file_name.split("__").collect::<Vec<&str>>();

                    let current_output_dir_path =
                        &output_dir_path.join(current_rdf_xml_file_name_splitted[0]);

                    fs::create_dir_all(&current_output_dir_path).unwrap();

                    std::fs::rename(
                        &current_path,
                        &current_output_dir_path.join(current_rdf_xml_file_name_splitted[1]),
                    )
                    .unwrap();
                }
            }
            Err(e) => {
                println!("  entry error: {:?}", e);
            }
        }
    }
}

#[test]
fn test_1() {
    let base_dir_path = std::env::current_dir().unwrap();
    let archive_file_path = base_dir_path.join("../../_data/expanded-main.zip");
    let output_dir_path = base_dir_path.join("../../_data/flat-tei-xml");

    unzip_files_to_single_dir(
        &archive_file_path,
        &output_dir_path,
        vec![".github", "test"],
    );
}

#[test]
fn test_2() {
    let base_dir_path = std::env::current_dir().unwrap();
    let input_dir_path = base_dir_path.join("../../_data/_tei-to-rdf-xml");
    let output_dir_path = base_dir_path.join("../../_data/rdf-xml");

    move_files_to_their_dirs(&input_dir_path, &output_dir_path);
}

#[test]
fn test_rdf_xml_to_turtle() {
    let base_dir_path = std::env::current_dir().unwrap();
    let input_dir_full_path = base_dir_path.join("../../_data/02-tei-to-rdf-xml");
    let output_dir_full_path = base_dir_path.join("../../_data/04-rdf-xml-to-rdf-turtle");

    //decompress_archive(input_dir_full_path, tmp_dir_full_path.clone());

    rdf_xml_to_turtle(&input_dir_full_path, &output_dir_full_path);
}

#[test]
fn test_rdf_turtle_to_rdf_xml() {
    let base_dir_path = std::env::current_dir().unwrap();
    let input_dir_full_path = base_dir_path.join("../../public/turtle/institutions/");
    let output_dir_full_path = base_dir_path.join("../../public/rdf/institutions/");

    rdf_turtle_to_rdf_xml(&input_dir_full_path, &output_dir_full_path);
}

/*fn decompress_archive(input_dir_full_path: PathBuf, tmp_dir_full_path: PathBuf) {
    let archives_glob = glob::Pattern::new("full*.zip").expect("wrong glob pattern for archives");
    let archive_globbed_path = input_dir_full_path.join(archives_glob.as_str());

    let mut archive_file_paths: Vec<PathBuf> = glob::glob(archive_globbed_path.to_str().unwrap())
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();
    archive_file_paths.sort();

    // select the latest archive
    let latest_archive_file_path = archive_file_paths.pop().unwrap();

    // copy the latest archive to the output folder and unzip it
    //fs::copy(latest_archive_file_path, output_dir_full_path).unwrap();
    let extract_opts = ExtractOptsBuilder::default()
        .strip(1)
        .filter(|path| {
            let path_as_str = path.to_str().unwrap();

            path_as_str.contains("/tmp/rdf/") && !path_as_str.ends_with("__contents__.xml")
            //
        })
        .build()
        .unwrap();
    decompress::decompress(latest_archive_file_path, tmp_dir_full_path, &extract_opts).unwrap();
}*/
