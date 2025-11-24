use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

use tokio::sync::mpsc::UnboundedSender;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
};

use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use super::Component;
use crate::{action::Action, config::Config};

// https://ratatui.rs/examples/widgets/tabs/


#[derive(Default)]
pub struct NavBar{
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Home")]
    Tab1,
    #[strum(to_string = "Notepad")]
    Tab2,
    #[strum(to_string = "Schedule")]
    Tab3,
    #[strum(to_string = "Settings")]
    Tab4,
}


impl NavBar {
    pub fn new() -> Self{
        Self::default()
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    // pub fn previous_tab(&mut self) {
    //     self.selected_tab = self.selected_tab.previous();
    // }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

impl Component for NavBar {
    fn register_action_handler(&mut self, tx:UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::NextTab => { 
                self.next_tab();
            }
            Action::Tick => {

            }
            Action::Render =>{

            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // frame.render_widget(Paragraph::new("Navbar"), area);
        frame.render_widget(self, area);
        Ok(())
    }
}


impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    // fn previous(self) -> Self {
    //     let current_index: usize = self as usize;
    //     let previous_index = current_index.saturating_sub(1);
    //     Self::from_repr(previous_index).unwrap_or(self)
    // }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let count = SelectedTab::iter().count();
        let next_index = (current_index + 1) % count;
        Self::from_repr(next_index).unwrap()
    }
}

impl Widget for &mut NavBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "yoyo v0.0.1".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("<tab> to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::ZINC,
            Self::Tab2 => tailwind::ZINC,
            Self::Tab3 => tailwind::ZINC,
            Self::Tab4 => tailwind::ZINC,
        }
    }
}
