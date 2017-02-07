use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;

/// Gets every json from a subdirectory and writes an html file in directory
/// to display the json data as a table, as well as create an index html
/// in directorys parentfolder containing a link to all created json tables
pub fn write_html_from_json(directory: &str, subdirectory: &str) {
    // The full path: dir\\subdir
    let mut full_path = directory.to_string();
    full_path.push_str("\\");
    full_path.push_str(subdirectory);
    // Create index file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("index.html")
        .unwrap();
    // Start of indexfile html
    let _ = writeln!(file, "<html><head></head><body>");
    // Get all json files
    // dir\\subdir\\<file>.json
    for entry in fs::read_dir(full_path.clone()).unwrap() {
        // Path to the file
        let dir = entry.unwrap();
        // Removes the subdirectory
        // dir\\<file>.json
        let path_parent = dir
            .path()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace(&format!("\\{}", subdirectory), "");
        // dir\\<file>.html
        let path_parent_html = path_parent.clone().replace("json", "html");
        // Create html file <dir>\\<file>.html
        let mut file_parent_html = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_parent_html.clone())
        .unwrap();
        // Write to dir\\<file>.html
        let _ = file_parent_html.write_all(

// LARGE HTML AHEAD
"<!DOCTYPE html>
<html>
    <head>
        <meta charset=\"utf-8\"/>
        <link rel=\"stylesheet\" href=\"css/jquery.dynatable.css\">
        <script type=\"text/javascript\" src=\"js/jquery-3.1.1.min.js\"></script>
        <script type=\"text/javascript\" src=\"js/jquery.dynatable.js\"></script>
    </head>
    <body>
        <a href=\"../index.html\">back</a><br>
        <table id=\"my-table\">
          <thead>
            <th>Artist</th>
            <th>Title</th>
            <th>Count</th>
          </thead>
          <tbody>
          </tbody>
        </table>
        <script>
            $.getJSON('"
// END OF LARGE HTML

        .as_bytes());
        // Link to subdir\\<file>.json
        let _ = file_parent_html.write_all(
            path_parent.replace(&directory, &subdirectory)
                .replace("\\", "/").as_bytes()
        );
        let _ = file_parent_html.write_all(

// MORE HTML
            "', function (response) {
                $('#my-table').dynatable({
                    dataset: {
                        records: response
                    },
                });
            });
        </script>
        <a href=\"../index.html\">back</a><br>
    </body>
</html>"
// END OF MORE HTML

        .as_bytes());
        // link to dir\\<file>.html
        let _ = writeln!(file, "<a href=\"{}?sorts[count]=-1\">{0}</a><br>", path_parent_html);
    }
    let _ = writeln!(file, "</body></html>");
}
