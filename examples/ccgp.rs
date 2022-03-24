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

    let mut count = 0;
    for page in website.get_pages() {
        let html = page.get_html();
        if let Some(totalnum) = html.select(&Selector::parse("#totalnum").unwrap()).next() {
            println!("totalnum: {}", totalnum.inner_html());
        } else if let Some(content) = html.select(&Selector::parse("#textarea").unwrap()).next() {
            if !content.inner_html().contains("项目编号：") {
                println!("{} not contains 项目编号：", page.get_url());
                continue;
            }
            count += 1;
            println!("{}----------------------------------------", count);
            for e in content.select(&Selector::parse("table tr td:only-child").unwrap()) {
                let s = e.inner_html();
                if s.contains("项目编号：") {
                    println!("项目编号 {}", parse_element_content("项目编号：", &e));
                } else if s.contains("项目名称：") {
                    println!("项目名称 {}", parse_element_content("项目名称：", &e));
                } else if s.contains("预算金额：") {
                    println!("预算金额 {}", parse_element_content("预算金额：", &e));
                } else if s.contains("开启时间：") {
                    println!("开启时间 {}", parse_element_content("开启时间：", &e));
                } else if s.contains("开启地点：") {
                    println!("开启地点 {}", parse_element_content("开启地点：", &e));
                } else if s.contains("截止时间：") {
                    println!("截止时间 {}", parse_element_content("截止时间：", &e));
                }
            }
        }
    }
}

fn parse_element_content(name: &str, element: &scraper::ElementRef) -> String {
    let s = element.inner_html().replace("&nbsp;", "");
    s.split(name).collect::<Vec<&str>>()[1].to_string()
}
