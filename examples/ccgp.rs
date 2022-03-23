use scraper::Selector;
use spider::website::Website;

fn main() {
    let mut website: Website = Website::new_start(
        "http://www.ccgp-shandong.gov.cn",
        "/sdgp2017/site/listnew.jsp?grade=province&colcode=0301",
    );
    website.configuration.verbose = true; // Defaults to false
    website.on_link_find_callback = |link| {
        if link.contains(&"/sdgp2017/site/listcontnew.jsp") {
            Some(link)
        } else {
            None
        }
    };
    website.crawl();

    for page in website.get_pages() {
        let html = page.get_html();
        if let Some(totalnum) = html.select(&Selector::parse("#totalnum").unwrap()).next() {
            println!("totalnum: {}", totalnum.inner_html());
        } else if let Some(content) = html.select(&Selector::parse("#textarea").unwrap()).next() {
            for e in content.select(&Selector::parse("table tr td:only-child").unwrap()) {
                let s = e.inner_html();
                if s.starts_with("&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;") {
                    println!("{}", s);
                }
            }
        }
    }
}
