current_dir="$PWD"
modules_dir=$current_dir/modules
generate_xar_package_dir=$modules_dir/generate-xar-package
config_dir=$generate_xar_package_dir/config

public_dir=$current_dir/public
expanded_data_dir=$public_dir/expanded-main
output_rdf_dir=$public_dir/rdf
output_turtle_dir=$public_dir/turtle

xar_package_name=bm-rdf.xar

ls -l $current_dir

exit 0;

mkdir -p $output_rdf_dir/{authority-files,institutions,manuscripts,narratives,persons,places,studies,works}
mkdir -p $output_turtle_dir/{authority-files,institutions,manuscripts,narratives,persons,places,studies,works}

echo "----- Download the archive with expanded data"
curl -L https://github.com/BetaMasaheft/expanded/archive/refs/heads/main.zip -o $public_dir/main.zip

echo "----- Unarchive the expanded data"
cd $public_dir && unzip -q main.zip && cd ..

echo "----- Process the files in 'authority-files' folder"
./data-utilities conceptual-mapper $config_dir/authority-files.toml
./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/authority-files/ $output_rdf_dir/authority-files/

echo "----- Process the files in 'institutions' folder"
./data-utilities conceptual-mapper $config_dir/institutions.toml
./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/institutions/ $output_rdf_dir/institutions/

echo "----- Process the files in 'manuscripts' folder"
./data-utilities conceptual-mapper $config_dir/manuscripts.toml
./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/manuscripts/ $output_rdf_dir/manuscripts/

echo "----- Process the files in 'narratives' folder"
./data-utilities conceptual-mapper $config_dir/narratives.toml
./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/narratives/ $output_rdf_dir/narratives/

echo "----- Process the files in 'persons' folder"
./data-utilities conceptual-mapper $config_dir/persons.toml
./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/persons/ $output_rdf_dir/persons/

echo "----- Process the files in 'places' folder"
./data-utilities conceptual-mapper $config_dir/places.toml
./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/places/ $output_rdf_dir/places/

echo "----- Process the files in 'studies' folder"
echo "----- Process the files in 'works' folder"
#./data-utilities conceptual-mapper $config_dir/studies.toml
#./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/studies/ $output_rdf_dir/studies/
# Remove this when using onceptual-mapper
flat_tei_files_dir=$public_dir/flat-tei-files
flat_rdf_files_dir=$public_dir/flat-rdf-files
mkdir -p $flat_tei_files_dir
mkdir -p $flat_rdf_files_dir
./data-utilities unzip-files-to-single-dir "$public_dir/main.zip" "$flat_tei_files_dir" config/ scripts/ test/ authority-files/ corpora/ institutions/ manuscripts/ narratives/ persons/ places/
saxonb-xslt -s:"$flat_tei_files_dir" -o:"$flat_rdf_files_dir" -xsl:"$generate_xar_package_dir/data2rdf.xsl"
for file in $flat_rdf_files_dir/*.xml ;
do
  mv "$file" "${file%.xml}.rdf"
done
./data-utilities move-files-to-their-dirs $flat_rdf_files_dir $output_rdf_dir
rm -rf $flat_tei_files_dir
rm -rf $flat_rdf_files_dir
# end of Remove this when using onceptual-mapper

#./data-utilities conceptual-mapper $config_dir/works.toml
#./data-utilities rdf-turtle-to-rdf-xml $output_turtle_dir/works/ $output_rdf_dir/works/

echo "Copy the xar package configuration files"
cp $generate_xar_package_dir/expath-pkg.xml $output_rdf_dir
cp $generate_xar_package_dir/repo.xml $output_rdf_dir

echo "Create the xar package"
cd $output_rdf_dir && zip -rq ../$xar_package_name . && cd ..

echo "Delete not needed files and folders"
rm $public_dir/main.zip
rm -rf $expanded_data_dir
rm -rf $output_turtle_dir
rm -rf $output_rdf_dir
