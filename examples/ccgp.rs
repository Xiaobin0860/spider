use scraper::Selector;
use spider::website::Website;

fn main() {
    let mut website: Website = Website::new_start(
        "http://www.ccgp-shandong.gov.cn",
        "/sdgp2017/site/listnew.jsp?grade=province&colcode=0301",
    );
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=2");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=3");
    website.configuration.verbose = true; // Defaults to false
    website.on_page_callback = Some(|page| {
        let html = page.get_html();
        if let Some(news) = html
            .select(&Selector::parse("ul.news_list2").unwrap())
            .next()
        {
            news.select(&Selector::parse("a").unwrap())
                .filter(|a| a.value().attrs().any(|attr| attr.0 == "href"))
                .map(|a| {
                    format!(
                        "http://www.ccgp-shandong.gov.cn{}",
                        a.value().attr("href").unwrap()
                    )
                })
                .collect()
        } else if let Some(content) = html.select(&Selector::parse("#noticeArea").unwrap()).next() {
            if content.inner_html().contains("项目编号：") {
                println!("----------------------------------------");
                for e in content.select(&Selector::parse("#noticeArea>p").unwrap()) {
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
            } else {
                println!("{} not contains 项目编号：", page.get_url());
            }
            Vec::new()
        } else if let Some(content) = html.select(&Selector::parse("#textarea").unwrap()).next() {
            if content.inner_html().contains("项目编号：") {
                println!("----------------------------------------");
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
            } else {
                println!("{} not contains 项目编号：", page.get_url());
            }
            Vec::new()
        } else {
            println!("{} no ul.news_list2 and #textarea", page.get_url());
            Vec::new()
        }
    });
    website.crawl();
}

fn parse_element_content(name: &str, element: &scraper::ElementRef) -> String {
    let mut s = String::new();
    element.text().for_each(|t| {
        s.push_str(t.trim());
    });
    let s = s.replace("&nbsp;", "");
    s.split(name).collect::<Vec<&str>>()[1].to_string()
}
