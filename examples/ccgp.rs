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
}
