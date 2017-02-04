// THIS IS THE UGLIEST THIGN EVER

pub fn write_html() {
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("index.html")
        .unwrap();
    writeln!(file, "<html><head></head><body>");
    for entry in fs::read_dir("data\\radio").unwrap() {
        let dir = entry.unwrap();
        let path = dir.path().into_os_string().into_string().unwrap().replace("\\radio", "");
        let mut path_2 = path.clone();
        path_2.push_str(".html");
        let mut file_2 = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_2)
        .unwrap();
        let _ = file_2.write_all(
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
            $.getJSON('".as_bytes());
        let _ = file_2.write_all(path.replace("data", "radio").replace("\\", "/").as_bytes());
        let _ = file_2.write_all(
// MORE HTML
            "', function (response) {
                $('#my-table').dynatable({
                    dataset: {
                        records: response
                    },
                });
            });
        </script>
    </body>
</html>
".as_bytes());
        writeln!(file, "<a href=\"{}.html\">{0}</a><br>", path);
    }
    writeln!(file, "</body></html>");
}
