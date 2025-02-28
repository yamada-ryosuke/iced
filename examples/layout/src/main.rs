use iced::border;
use iced::keyboard;
use iced::mouse;
use iced::widget::{
    button, canvas, center, center_y, checkbox, column, container,
    horizontal_rule, horizontal_space, pick_list, pin, row, scrollable, stack,
    text, vertical_rule,
};
use iced::{
    Center, Element, Fill, Font, Length, Point, Rectangle, Renderer, Shrink,
    Subscription, Theme, color,
};

pub fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}

// ElmアーキテクチャのStateに当たる構造体。
#[derive(Default, Debug)]
struct Layout {
    // 現在のページ。
    page: Page,
    // 各widgetがどういう形をしているか分かりやすくするための枠を表示するか否かのステータス。
    explain: bool,
    // アプリのテーマカラー。
    theme: Theme,
}

// ElmアーキテクチャのMessageに当たる構造体。
#[derive(Debug, Clone)]
enum Message {
    // 次のページへ。
    Next,
    // 前のページへ。
    Previous,
    // trueならばLayoutのexplainをオン。falseならばオフ。
    ExplainToggled(bool),
    // テーマカラーの選択。
    ThemeSelected(Theme),
}

impl Layout {
    // main関数でapplicationに渡してタイトルを表示するために用いられる。
    fn title(&self) -> String {
        format!("{} - Layout - Iced", self.page.title)
    }

    // ElmアーキテクチャのUpdate Logicに当たる構造体。
    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.page = self.page.next();
            }
            Message::Previous => {
                self.page = self.page.previous();
            }
            Message::ExplainToggled(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    // キーボードの左右ボタンで次のページや前のページに遷移できる。
    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_release(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Previous),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    // ElmアーキテクチャのView Logicに当たる構造体。
    fn view(&self) -> Element<Message> {
        let header = row![
            text(self.page.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain)
                .on_toggle(Message::ExplainToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_y(Center);

        let example = center(if self.explain {
            self.page.view().explain(color!(0x0000ff))
        } else {
            self.page.view()
        })
        .style(|theme| {
            let palette = theme.extended_palette();

            container::Style::default()
                .border(border::color(palette.background.strong.color).width(4))
        })
        .padding(4);

        let controls = row([
            (!self.page.is_first()).then_some(
                button("← Previous")
                    .padding([5, 10])
                    .on_press(Message::Previous)
                    .into(),
            ),
            Some(horizontal_space().into()),
            (!self.page.is_last()).then_some(
                button("Next →")
                    .padding([5, 10])
                    .on_press(Message::Next)
                    .into(),
            ),
        ]
        .into_iter()
        .flatten());

        column![header, example, controls]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

// ページの構造体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Page {
    title: &'static str,

    // 具体的な表示内容を表すElementを返す関数をメンバとして持っている。
    view: fn() -> Element<'static, Message>,
}

impl Page {
    // このページが最初のページか？
    fn is_first(self) -> bool {
        AllPage::is_first(&self)
    }

    // このページが最後のページか？
    fn is_last(self) -> bool {
        AllPage::is_last(&self)
    }

    // 一つ前のページを検索する。
    fn previous(self) -> Page {
        AllPage::previous(&self)
    }

    // 一つ後のページを検索する。
    fn next(self) -> Page {
        AllPage::next(&self)
    }

    // viewメンバをメソッドとしてラッピングしている
    fn view(&self) -> Element<Message> {
        (self.view)()
    }
}

impl Default for Page {
    fn default() -> Self {
        AllPage::LIST[0]
    }
}

// ページの全体に関する情報を扱う構造体。
struct AllPage {}

impl AllPage {
    // このアプリで表示可能な全てのページを前から順に並べたリスト。
    const LIST: &'static [Page] = &[
        Page {
            title: "Centered",
            view: centered,
        },
        Page {
            title: "Column",
            view: column_,  // おそらくicedのcolumnマクロと名前が被るからアンダーバーがついてる
        },
        Page {
            title: "Row",
            view: row_,     // おそらくicedのrowマクロと名前が被るからアンダーバーがついてる
        },
        Page {
            title: "Space",
            view: space,
        },
        Page {
            title: "Application",
            view: application,
        },
        Page {
            title: "Quotes",
            view: quotes,
        },
        Page {
            title: "Pinning",
            view: pinning,
        },
        
    ];

    // 現在のページが最初のページか？
    fn is_first(page: &Page) -> bool {
        AllPage::LIST.first() == Some(&page)
    }

    // 現在のページが最後のページか？
    fn is_last(page: &Page) -> bool {
        AllPage::LIST.last() == Some(&page)
    }

    // 一つ前のページを検索する。
    fn previous(page: &Page) -> Page {
        let Some(index) =
            AllPage::LIST.iter().position(|&example| example == *page)
        else {
            return *page;
        };

        AllPage::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(*page)
    }

    // 一つ後のページを検索する。
    fn next(page: &Page) -> Page {
        let Some(index) =
            AllPage::LIST.iter().position(|&example| example == *page)
        else {
            return *page;
        };

        AllPage::LIST.get(index + 1).copied().unwrap_or(*page)
    }

}

// 以下、各Pageのviewメンバに入れる関数
fn centered<'a>() -> Element<'a, Message> {
    center(text("I am centered!").size(50)).into()
}

fn column_<'a>() -> Element<'a, Message> {
    column![
        "A column can be used to",
        "lay out widgets vertically.",
        square(50),
        square(50),
        square(50),
        "The amount of space between",
        "elements can be configured!",
    ]
    .spacing(40)
    .into()
}

fn row_<'a>() -> Element<'a, Message> {
    row![
        "A row works like a column...",
        square(50),
        square(50),
        square(50),
        "but lays out widgets horizontally!",
    ]
    .spacing(40)
    .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn application<'a>() -> Element<'a, Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(),
            "Header!",
            horizontal_space(),
            square(40),
        ]
        .padding(10)
        .align_y(Center),
    )
    .style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default()
            .border(border::color(palette.background.strong.color).width(1))
    });

    let sidebar = center_y(
        column!["Sidebar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_x(Center),
    )
    .style(container::rounded_box);

    let content = container(
        scrollable(
            column![
                "Content!",
                row((1..10).map(|i| square(if i % 2 == 0 { 80 } else { 160 })))
                    .spacing(20)
                    .align_y(Center)
                    .wrap(),
                "The end"
            ]
            .spacing(40)
            .align_x(Center)
            .width(Fill),
        )
        .height(Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}

fn quotes<'a>() -> Element<'a, Message> {
    fn quote<'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        row![vertical_rule(2), content.into()]
            .spacing(10)
            .height(Shrink)
            .into()
    }

    fn reply<'a>(
        original: impl Into<Element<'a, Message>>,
        reply: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        column![quote(original), reply.into()].spacing(10).into()
    }

    column![
        reply(
            reply("This is the original message", "This is a reply"),
            "This is another reply",
        ),
        horizontal_rule(1),
        "A separator ↑",
    ]
    .width(Shrink)
    .spacing(10)
    .into()
}

fn pinning<'a>() -> Element<'a, Message> {
    column![
        "The pin widget can be used to position a widget \
        at some fixed coordinates inside some other widget.",
        stack![
            container(pin("• (50, 50)").x(50).y(50))
                .width(500)
                .height(500)
                .style(container::bordered_box),
            pin("• (300, 300)").x(300).y(300),
        ]
    ]
    .align_x(Center)
    .spacing(10)
    .into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
