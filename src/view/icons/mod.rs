use maud::{html, Markup};

pub fn magnifying_glass(fill: String, size: i32) -> Markup {
    html! {
        svg class="inline-block" xmlns="http://www.w3.org/2000/svg" width=(size) height=(size) fill=(fill) viewBox="0 0 256 256" {
            path d="M229.66,218.34l-50.07-50.06a88.11,88.11,0,1,0-11.31,11.31l50.06,50.07a8,8,0,0,0,11.32-11.32ZM40,112a72,72,0,1,1,72,72A72.08,72.08,0,0,1,40,112Z" {}
        }
    }
}

pub fn bird(fill: String, size: i32) -> Markup {
    html! {
        svg class="inline-block" xmlns="http://www.w3.org/2000/svg" width=(size) height=(size) fill=(fill) viewBox="0 0 256 256" style="--darkreader-inline-fill: #000000;" data-darkreader-inline-fill="" {
            path d="M176,72a16,16,0,1,1-16-16A16,16,0,0,1,176,72Zm68,8a12,12,0,0,1-5.34,10L220,102.42V120A108.12,108.12,0,0,1,112,228H24A20,20,0,0,1,8.41,195.5l.15-.18L92,95.18V76.89C92,41.28,120.57,12.17,155.69,12H156a63.94,63.94,0,0,1,60.58,43.29L238.66,70A12,12,0,0,1,244,80Zm-33.63,0-10.69-7.13a12,12,0,0,1-5-7A40,40,0,0,0,156,36h-.19c-21.95.11-39.8,18.45-39.8,40.89V99.52a12,12,0,0,1-2.79,7.69L32.57,204H53.05l69.74-83.68a12,12,0,1,1,18.43,15.36L84.29,204H112a84.09,84.09,0,0,0,84-84V96a12,12,0,0,1,5.35-10Z" {}
        }
    }
}
