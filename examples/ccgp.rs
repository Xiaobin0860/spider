use anyhow::{Ok, Result};
use scraper::Selector;
use spider::{
    page::Page,
    website::{PageHandler, Website},
};
use tracing::{debug, error, info, warn};
use xlsxwriter::*;

const HEADER: [&str; 6] = [
    "项目编号",
    "项目名称",
    "预算金额",
    "开启时间",
    "开启地点",
    "截止时间",
];

struct MyPageHandler {
    book: Workbook,
    row: u32,
}

impl MyPageHandler {
    fn new(book: Workbook) -> Self {
        Self { book, row: 1 }
    }

    fn write_row(&mut self, values: &[String]) {
        let mut sheet = self.book.get_worksheet("sheet").unwrap();
        for (i, value) in values.iter().enumerate() {
            match sheet.write_string(self.row, i as u16, value, None) {
                Err(e) => error!("{}", e),
                _ => debug!("write {}:{} {}", self.row, i, value),
            }
        }
        self.row += 1;
    }
}
impl PageHandler for MyPageHandler {
    fn handle(&mut self, page: Page) -> Vec<String> {
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
            let content_inner = content.inner_html();
            if content_inner.contains("编号：") || content_inner.contains("编号）：") {
                let vs = parse_project_info(&content, "#noticeArea>p");
                self.write_row(&vs);
            } else {
                println!("{} #noticeArea not contains 项目编号", page.get_url());
            }
            Vec::new()
        } else if let Some(content) = html.select(&Selector::parse("#textarea").unwrap()).next() {
            let content_inner = content.inner_html();
            if content_inner.contains("编号：") || content_inner.contains("编号）：") {
                let vs = parse_project_info(&content, "table tr td:only-child");
                self.write_row(&vs);
            } else {
                warn!("{} #textarea not contains 项目编号", page.get_url());
            }
            Vec::new()
        } else {
            warn!(
                "{} no 'ul.news_list2', '#noticeArea' or '#textarea'",
                page.get_url()
            );
            Vec::new()
        }
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let wb = Workbook::new("result.xlsx");
    let mut sheet = wb.add_worksheet(Some("sheet")).unwrap();
    for (i, v) in HEADER.into_iter().enumerate() {
        sheet.write_string(0, i as u16, v, None).unwrap();
    }

    let mut website = Website::new_start(
        "http://www.ccgp-shandong.gov.cn",
        "/sdgp2017/site/listnew.jsp?grade=province&colcode=0301",
    );
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=2");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=50");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=100");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=200");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=300");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=400");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=500");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=600");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=700");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=800");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=900");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=1000");
    website.add_link("http://www.ccgp-shandong.gov.cn/sdgp2017/site/listnew.jsp?grade=province&colcode=0301&curpage=1&projectcode=SDGP370000201902007131");
    website.configuration.verbose = true; // Defaults to false
    website.page_handler = Some(Box::new(MyPageHandler::new(wb)));
    website.crawl();

    info!("crawl finished, save result to result.xlsx");

    Ok(())
}

fn parse_element_content(name: &str, element: &scraper::ElementRef) -> String {
    let mut s = String::new();
    element.text().for_each(|t| {
        s.push_str(t.trim());
    });
    let s = s.replace("&nbsp;", "");
    s.split(name).collect::<Vec<&str>>()[1].to_string()
}

fn parse_project_info(content: &scraper::ElementRef, selector: &str) -> Vec<String> {
    debug!("----------------------------------------");
    let mut values = vec!["".to_string(); 6];
    for e in content.select(&Selector::parse(selector).unwrap()) {
        let s = e.inner_html();
        if s.contains("编号：") || s.contains("编号）：") {
            let v = parse_element_content("：", &e);
            debug!("项目编号 {}", v);
            values[0] = v;
        } else if s.contains("项目名称：") {
            let v = parse_element_content("项目名称：", &e);
            debug!("项目名称 {}", v);
            values[1] = v;
        } else if s.contains("预算金额：") {
            let v = parse_element_content("预算金额：", &e);
            debug!("预算金额 {}", v);
            values[2] = v;
        } else if s.contains("开启时间：") {
            let v = parse_element_content("开启时间：", &e);
            debug!("开启时间 {}", v);
            values[3] = v;
        } else if s.contains("开启地点：") {
            let v = parse_element_content("开启地点：", &e);
            debug!("开启地点 {}", v);
            values[4] = v;
        } else if s.contains("截止时间：") {
            let v = parse_element_content("截止时间：", &e);
            debug!("截止时间 {}", v);
            values[5] = v;
        }
    }
    values
}
