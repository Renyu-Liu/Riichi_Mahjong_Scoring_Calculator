// gui.rs: Iced GUI

use crate::implements::calculate_agari;
use crate::implements::game::{AgariType, GameContext, PlayerContext};
use crate::implements::hand::MentsuType;
use crate::implements::input::{OpenMeldInput, UserInput};
use crate::implements::tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai};
use iced::widget::{button, checkbox, column, container, image, radio, row, scrollable, text};
use iced::{Color, Element, Length, Sandbox, Settings, theme};

pub fn run() -> iced::Result {
    RiichiGui::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    // Phase 1 Messages
    AddTile(Hai),
    RemoveTile(usize),
    ConfirmHand,
    ModifyHand,
    // Phase 2 Messages
    StartSelectWinningTile,
    SelectWinningTile(Hai),
    StartAddOpenMeld,
    SelectMeldType(MentsuType),
    SelectMeldTile(Hai),
    StartAddClosedKan,
    SelectClosedKan(Hai),
    EditOpenMeld(usize),
    EditClosedKan(usize),
    RemoveOpenMeld(usize),
    RemoveClosedKan(usize),
    // Phase 3 Messages
    ToggleAgariType(AgariType),
    SetBakaze(Kaze),
    SetJikaze(Kaze),
    ToggleRiichi(bool),
    ToggleDoubleRiichi(bool),
    ToggleIppatsu(bool),
    ToggleRinshan(bool),
    ToggleChankan(bool),
    ToggleHaitei(bool),
    ToggleHoutei(bool),
    ToggleTenhou(bool),
    ToggleChiihou(bool),
    ToggleRenhou(bool),
    IncrementHonba,
    DecrementHonba,
    StartAddDora,
    SelectDora(Hai),
    StartAddUraDora,
    SelectUraDora(Hai),
    CalculateScore,
    StartOver,
    // Akadora Messages
    IncrementAkadora,
    DecrementAkadora,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Phase {
    Composition,
    Definition,
    SelectingWinningTile,
    SelectingMeldType { editing_index: Option<usize> },
    SelectingMeldTile(MentsuType, Option<usize>),
    SelectingClosedKan { editing_index: Option<usize> },
    SelectingDora,
    SelectingUraDora,
    Result,
}

struct RiichiGui {
    phase: Phase,

    // Phase 1 State
    hand_tiles: Vec<Hai>,
    tile_counts: [u8; 34],

    // Phase 2 State
    winning_tile: Option<Hai>,
    open_melds: Vec<OpenMeldInput>,
    closed_kans: Vec<Hai>,

    // Phase 3 State
    agari_type: AgariType,
    bakaze: Kaze,
    jikaze: Kaze,
    is_riichi: bool,
    is_daburu_riichi: bool,
    is_ippatsu: bool,
    is_rinshan: bool,
    is_chankan: bool,
    is_haitei: bool,
    is_houtei: bool,
    is_tenhou: bool,
    is_chiihou: bool,
    is_renhou: bool,
    honba: u8,
    num_akadora: u8,
    dora_indicators: Vec<Hai>,
    uradora_indicators: Vec<Hai>,

    score_result: Option<Result<crate::implements::scoring::AgariResult, String>>,
}

impl Sandbox for RiichiGui {
    type Message = Message;

    fn new() -> Self {
        Self {
            phase: Phase::Composition,
            hand_tiles: Vec::new(),
            tile_counts: [4; 34],
            winning_tile: None,
            open_melds: Vec::new(),
            closed_kans: Vec::new(),
            agari_type: AgariType::Ron,
            bakaze: Kaze::Ton,
            jikaze: Kaze::Ton,
            is_riichi: false,
            is_daburu_riichi: false,
            is_ippatsu: false,
            is_rinshan: false,
            is_chankan: false,
            is_haitei: false,
            is_houtei: false,
            is_tenhou: false,
            is_chiihou: false,
            is_renhou: false,
            honba: 0,
            num_akadora: 0,
            dora_indicators: Vec::new(),
            uradora_indicators: Vec::new(),
            score_result: None,
        }
    }

    fn title(&self) -> String {
        String::from("Riichi Mahjong Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddTile(tile) => {
                if self.hand_tiles.len() < 18 {
                    let idx = crate::implements::tiles::tile_to_index(&tile);
                    if self.tile_counts[idx] > 0 {
                        self.tile_counts[idx] -= 1;
                        self.hand_tiles.push(tile);
                        self.hand_tiles.sort_by_key(Self::sort_tiles_by_type);
                    }
                }
            }
            Message::RemoveTile(index) => {
                if index < self.hand_tiles.len() {
                    let tile = self.hand_tiles.remove(index);
                    let idx = crate::implements::tiles::tile_to_index(&tile);
                    self.tile_counts[idx] += 1;
                }
            }
            Message::ConfirmHand => {
                if self.hand_tiles.len() >= 14 {
                    self.phase = Phase::Definition;
                }
            }
            Message::ModifyHand => {
                self.phase = Phase::Composition;
                self.winning_tile = None;
                self.open_melds.clear();
                self.closed_kans.clear();
                self.num_akadora = 0;
            }
            Message::StartSelectWinningTile => {
                self.phase = Phase::SelectingWinningTile;
            }
            Message::SelectWinningTile(tile) => {
                self.winning_tile = Some(tile);
                self.phase = Phase::Definition;
            }
            Message::StartAddOpenMeld => {
                self.phase = Phase::SelectingMeldType {
                    editing_index: None,
                };
            }
            Message::SelectMeldType(m_type) => {
                let editing_idx = if let Phase::SelectingMeldType { editing_index } = self.phase {
                    editing_index
                } else {
                    None
                };
                self.phase = Phase::SelectingMeldTile(m_type, editing_idx);
            }
            Message::SelectMeldTile(tile) => {
                if let Phase::SelectingMeldTile(m_type, editing_idx) = self.phase {
                    let meld = OpenMeldInput {
                        mentsu_type: m_type,
                        representative_tile: tile,
                    };
                    if let Some(idx) = editing_idx {
                        if idx < self.open_melds.len() {
                            self.open_melds[idx] = meld;
                        }
                    } else {
                        self.open_melds.push(meld);
                    }

                    if !self.open_melds.is_empty() {
                        self.is_riichi = false;
                        self.is_daburu_riichi = false;
                        self.is_ippatsu = false;
                        self.is_tenhou = false;
                        self.is_chiihou = false;
                        self.is_renhou = false;
                    }
                }
                self.phase = Phase::Definition;
            }
            Message::StartAddClosedKan => {
                self.phase = Phase::SelectingClosedKan {
                    editing_index: None,
                };
            }
            Message::SelectClosedKan(tile) => {
                let editing_idx = if let Phase::SelectingClosedKan { editing_index } = self.phase {
                    editing_index
                } else {
                    None
                };

                if let Some(idx) = editing_idx {
                    if idx < self.closed_kans.len() {
                        self.closed_kans[idx] = tile;
                    }
                } else {
                    self.closed_kans.push(tile);
                }
                self.phase = Phase::Definition;
            }
            Message::EditOpenMeld(idx) => {
                if idx < self.open_melds.len() {
                    self.phase = Phase::SelectingMeldType {
                        editing_index: Some(idx),
                    };
                }
            }
            Message::EditClosedKan(idx) => {
                if idx < self.closed_kans.len() {
                    self.phase = Phase::SelectingClosedKan {
                        editing_index: Some(idx),
                    };
                }
            }
            Message::RemoveOpenMeld(idx) => {
                if idx < self.open_melds.len() {
                    self.open_melds.remove(idx);
                }
            }
            Message::RemoveClosedKan(idx) => {
                if idx < self.closed_kans.len() {
                    self.closed_kans.remove(idx);
                }
            }
            Message::ToggleAgariType(agari_type) => {
                self.agari_type = agari_type;
                match self.agari_type {
                    AgariType::Ron => {
                        self.is_tenhou = false;
                        self.is_chiihou = false;
                        self.is_haitei = false;
                        self.is_rinshan = false;
                    }
                    AgariType::Tsumo => {
                        self.is_renhou = false;
                        self.is_houtei = false;
                        self.is_chankan = false;
                    }
                }
            }
            Message::SetBakaze(kaze) => {
                self.bakaze = kaze;
            }
            Message::SetJikaze(kaze) => {
                self.jikaze = kaze;
                if self.jikaze != Kaze::Ton {
                    self.is_tenhou = false;
                } else {
                    self.is_chiihou = false;
                }
            }
            Message::ToggleRiichi(is_riichi) => {
                self.is_riichi = is_riichi;
                if !self.is_riichi {
                    self.is_daburu_riichi = false;
                    self.is_ippatsu = false;
                }
            }
            Message::ToggleDoubleRiichi(is_double) => {
                self.is_daburu_riichi = is_double;
                if self.is_daburu_riichi {
                    self.is_riichi = true;
                }
            }
            Message::ToggleIppatsu(is_ippatsu) => {
                if self.is_riichi {
                    self.is_ippatsu = is_ippatsu;
                }
            }
            Message::ToggleRinshan(val) => self.is_rinshan = val,
            Message::ToggleChankan(val) => self.is_chankan = val,
            Message::ToggleHaitei(val) => self.is_haitei = val,
            Message::ToggleHoutei(val) => self.is_houtei = val,
            Message::ToggleTenhou(val) => self.is_tenhou = val,
            Message::ToggleChiihou(val) => self.is_chiihou = val,
            Message::ToggleRenhou(val) => self.is_renhou = val,
            Message::IncrementHonba => self.honba += 1,
            Message::DecrementHonba => {
                if self.honba > 0 {
                    self.honba -= 1
                }
            }
            Message::StartAddDora => self.phase = Phase::SelectingDora,
            Message::SelectDora(tile) => {
                self.dora_indicators.push(tile);
                self.phase = Phase::Definition;
            }
            Message::StartAddUraDora => self.phase = Phase::SelectingUraDora,
            Message::SelectUraDora(tile) => {
                self.uradora_indicators.push(tile);
                self.phase = Phase::Definition;
            }
            Message::CalculateScore => {
                // Collected inputs
                if let Some(winning_tile) = self.winning_tile {
                    // Remove tiles used in Open Melds
                    let mut hand_tiles = self.hand_tiles.clone();
                    for meld in &self.open_melds {
                        let tiles = self.get_meld_tiles(meld);
                        for t in tiles {
                            if let Some(pos) = hand_tiles.iter().position(|x| x == &t) {
                                hand_tiles.remove(pos);
                            }
                        }
                    }

                    // Remove tiles used in Closed Kans
                    for kan in &self.closed_kans {
                        for _ in 0..4 {
                            if let Some(pos) = hand_tiles.iter().position(|x| x == kan) {
                                hand_tiles.remove(pos);
                            }
                        }
                    }

                    // remove the winning tile from hand
                    if self.agari_type == AgariType::Ron {
                        if let Some(pos) = hand_tiles.iter().position(|x| x == &winning_tile) {
                            hand_tiles.remove(pos);
                        }
                    }

                    let final_hand_tiles: Vec<Hai> = hand_tiles;
                    let final_open_melds: Vec<OpenMeldInput> = self.open_melds.clone();
                    let final_closed_kans: Vec<Hai> = self.closed_kans.clone();

                    let input = UserInput {
                        hand_tiles: final_hand_tiles,
                        open_melds: final_open_melds,
                        closed_kans: final_closed_kans,
                        winning_tile,
                        agari_type: self.agari_type,
                        player_context: PlayerContext {
                            jikaze: self.jikaze,
                            is_oya: self.jikaze == Kaze::Ton,
                            is_riichi: self.is_riichi,
                            is_daburu_riichi: self.is_daburu_riichi,
                            is_ippatsu: self.is_ippatsu,
                            is_menzen: self.open_melds.is_empty(),
                        },
                        game_context: GameContext {
                            bakaze: self.bakaze,
                            kyoku: 1,
                            honba: self.honba,
                            riichi_bou: if self.is_riichi { 1 } else { 0 },
                            dora_indicators: self.dora_indicators.clone(),
                            uradora_indicators: self.uradora_indicators.clone(),
                            num_akadora: self.num_akadora,
                            is_tenhou: self.is_tenhou,
                            is_chiihou: self.is_chiihou,
                            is_renhou: self.is_renhou,
                            is_haitei: self.is_haitei,
                            is_houtei: self.is_houtei,
                            is_rinshan: self.is_rinshan,
                            is_chankan: self.is_chankan,
                        },
                    };
                    // Calculate score
                    match calculate_agari(&input) {
                        Ok(result) => {
                            self.score_result = Some(Ok(result));
                        }
                        Err(e) => {
                            self.score_result = Some(Err(format!("Error: {}", e)));
                        }
                    }
                    self.phase = Phase::Result;
                }
            }
            Message::StartOver => {
                *self = Self::new();
            }
            Message::IncrementAkadora => {
                if self.num_akadora < 4 {
                    self.num_akadora += 1;
                }
            }
            Message::DecrementAkadora => {
                if self.num_akadora > 0 {
                    self.num_akadora -= 1;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = match &self.phase {
            Phase::Composition => self.view_composition(),
            Phase::Definition => self.view_definition(),
            Phase::SelectingWinningTile => self.view_selecting_winning_tile(),
            Phase::SelectingMeldType { .. } => self.view_selecting_meld_type(),
            Phase::SelectingMeldTile(m_type, _) => self.view_selecting_meld_tile(*m_type),
            Phase::SelectingClosedKan { .. } => self.view_selecting_closed_kan(),
            Phase::SelectingDora => self.view_selecting_dora(false),
            Phase::SelectingUraDora => self.view_selecting_dora(true),
            Phase::Result => self.view_result(),
        };

        container(scrollable(container(content).width(Length::Fill).center_x()).width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

impl RiichiGui {
    /// Helper function to sort tiles by type first, then by number
    /// Returns a tuple (type_order, number) for sorting
    fn sort_tiles_by_type(tile: &Hai) -> (u8, u8) {
        match tile {
            // Man tiles: type order 0, sorted by number 1-9
            Hai::Suhai(n, Suhai::Manzu) => (0, *n),
            // Pin tiles: type order 1, sorted by number 1-9
            Hai::Suhai(n, Suhai::Pinzu) => (1, *n),
            // Sou tiles: type order 2, sorted by number 1-9
            Hai::Suhai(n, Suhai::Souzu) => (2, *n),
            // Wind tiles: type order 3, sorted by Ton, Nan, Shaa, Pei
            Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => (3, 0),
            Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => (3, 1),
            Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => (3, 2),
            Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => (3, 3),
            // Dragon tiles: type order 4, sorted by Haku, Hatsu, Chun
            Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => (4, 0),
            Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => (4, 1),
            Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => (4, 2),
        }
    }
    fn view_composition(&self) -> Element<'_, Message> {
        let hand_preview = self.view_hand_preview();
        let tile_pool = self.view_tile_pool();

        let tile_count = self.hand_tiles.len();
        let counter_color = if tile_count < 14 {
            Color::from_rgb(0.8, 0.0, 0.0)
        } else {
            Color::from_rgb(0.0, 0.5, 0.0)
        };

        let counter_text = text(format!("Your Hand ({}/18 tiles)", tile_count))
            .size(20)
            .style(counter_color);

        let confirm_btn = button(text("Confirm Hand"))
            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                background_color: Color::from_rgb(0.0, 0.6, 0.0),
                text_color: Color::WHITE,
            })))
            .on_press_maybe(if tile_count >= 14 {
                Some(Message::ConfirmHand)
            } else {
                None
            });

        column![counter_text, hand_preview, confirm_btn, tile_pool]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn view_definition(&self) -> Element<'_, Message> {
        let hand_preview = self.view_hand_preview_locked();
        let modify_btn = button(text("Modify Hand")).on_press(Message::ModifyHand);

        let winning_tile_section = column![
            text("Winning Tile:"),
            match &self.winning_tile {
                Some(t) => {
                    {
                        let e: Element<Message> = row![
                            iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t, false)
                            )
                            .width(40),
                            button(text("Change")).on_press(Message::StartSelectWinningTile)
                        ]
                        .spacing(10)
                        .align_items(iced::Alignment::Center)
                        .into();
                        e
                    }
                }
                None =>
                    row![button(text("Select")).on_press(Message::StartSelectWinningTile)].into(),
            }
        ]
        .spacing(10);

        let melds_section = column![
            text("Melds:"),
            column(
                self.open_melds
                    .iter()
                    .enumerate()
                    .map(|(i, m)| {
                        let tiles = self.get_meld_tiles(m);
                        let tile_images = row(tiles
                            .iter()
                            .map(|t| {
                                iced::widget::Image::<iced::widget::image::Handle>::new(
                                    get_tile_image_path(t, false),
                                )
                                .width(40)
                                .into()
                            })
                            .collect::<Vec<Element<Message>>>())
                        .spacing(2);

                        row![
                            tile_images,
                            button(text("Change")).on_press(Message::EditOpenMeld(i)),
                            button(text("Remove")).on_press(Message::RemoveOpenMeld(i))
                        ]
                        .spacing(10)
                        .align_items(iced::Alignment::Center)
                        .into()
                    })
                    .collect::<Vec<Element<Message>>>()
            )
            .spacing(10),
            column(
                self.closed_kans
                    .iter()
                    .enumerate()
                    .map(|(i, k)| {
                        let tiles = vec![*k; 4];
                        let tile_images = row(tiles
                            .iter()
                            .map(|t| {
                                iced::widget::Image::<iced::widget::image::Handle>::new(
                                    get_tile_image_path(t, false),
                                )
                                .width(40)
                                .into()
                            })
                            .collect::<Vec<Element<Message>>>())
                        .spacing(2);

                        row![
                            tile_images,
                            button(text("Change")).on_press(Message::EditClosedKan(i)),
                            button(text("Remove")).on_press(Message::RemoveClosedKan(i))
                        ]
                        .spacing(10)
                        .align_items(iced::Alignment::Center)
                        .into()
                    })
                    .collect::<Vec<Element<Message>>>()
            )
            .spacing(10),
            row![
                button(text("Add Open Meld")).on_press(Message::StartAddOpenMeld),
                button(text("Add Closed Kan")).on_press(Message::StartAddClosedKan)
            ]
            .spacing(10)
        ]
        .spacing(10);

        let is_oya = self.jikaze == Kaze::Ton;
        let is_ron = self.agari_type == AgariType::Ron;
        let is_tsumo = self.agari_type == AgariType::Tsumo;
        let is_menzen = self.open_melds.is_empty();

        // conflicts
        let checkbox_with_conflict = |label: &str,
                                      is_checked: bool,
                                      msg: fn(bool) -> Message,
                                      is_enabled: bool|
         -> Element<Message> {
            if is_enabled {
                checkbox(label, is_checked).on_toggle(msg).into()
            } else {
                row![
                    checkbox("", is_checked),
                    text(label).style(Color::from_rgb(0.5, 0.5, 0.5))
                ]
                .spacing(0)
                .align_items(iced::Alignment::Center)
                .into()
            }
        };

        let context_section = column![
            text("Game Context:").size(18),
            // Agari Type
            row![
                text("Win Type:"),
                radio(
                    "Ron",
                    AgariType::Ron,
                    Some(self.agari_type),
                    Message::ToggleAgariType
                ),
                radio(
                    "Tsumo",
                    AgariType::Tsumo,
                    Some(self.agari_type),
                    Message::ToggleAgariType
                ),
            ]
            .spacing(20),
            // Bakaze
            row![
                text("Prevalent Wind (Bakaze):"),
                radio("East", Kaze::Ton, Some(self.bakaze), Message::SetBakaze),
                radio("South", Kaze::Nan, Some(self.bakaze), Message::SetBakaze),
                radio("West", Kaze::Shaa, Some(self.bakaze), Message::SetBakaze),
                radio("North", Kaze::Pei, Some(self.bakaze), Message::SetBakaze),
            ]
            .spacing(10),
            // Jikaze
            row![
                text("Player Wind (Jikaze):"),
                radio("East", Kaze::Ton, Some(self.jikaze), Message::SetJikaze),
                radio("South", Kaze::Nan, Some(self.jikaze), Message::SetJikaze),
                radio("West", Kaze::Shaa, Some(self.jikaze), Message::SetJikaze),
                radio("North", Kaze::Pei, Some(self.jikaze), Message::SetJikaze),
            ]
            .spacing(10),
            // Riichi & Status
            row![
                checkbox_with_conflict("Riichi", self.is_riichi, Message::ToggleRiichi, is_menzen),
                checkbox_with_conflict(
                    "Double Riichi",
                    self.is_daburu_riichi,
                    Message::ToggleDoubleRiichi,
                    is_menzen
                ),
                checkbox_with_conflict(
                    "Ippatsu",
                    self.is_ippatsu,
                    Message::ToggleIppatsu,
                    self.is_riichi || self.is_daburu_riichi
                ),
            ]
            .spacing(10),
            // Special Yaku
            column![
                text("Special Yaku:"),
                row![
                    checkbox_with_conflict(
                        "Tenhou",
                        self.is_tenhou,
                        Message::ToggleTenhou,
                        is_tsumo && is_oya && is_menzen
                    ),
                    checkbox_with_conflict(
                        "Chiihou",
                        self.is_chiihou,
                        Message::ToggleChiihou,
                        is_tsumo && !is_oya && is_menzen
                    ),
                    checkbox_with_conflict(
                        "Renhou",
                        self.is_renhou,
                        Message::ToggleRenhou,
                        is_ron && is_menzen
                    ),
                ]
                .spacing(10),
                row![
                    checkbox_with_conflict(
                        "Haitei",
                        self.is_haitei,
                        Message::ToggleHaitei,
                        is_tsumo
                    ),
                    checkbox_with_conflict("Houtei", self.is_houtei, Message::ToggleHoutei, is_ron),
                    checkbox_with_conflict(
                        "Rinshan",
                        self.is_rinshan,
                        Message::ToggleRinshan,
                        is_tsumo
                    ),
                    checkbox_with_conflict(
                        "Chankan",
                        self.is_chankan,
                        Message::ToggleChankan,
                        is_ron
                    ),
                ]
                .spacing(10),
            ]
            .spacing(5),
            row![
                text(format!("Honba: {}", self.honba)),
                button(text("+")).on_press(Message::IncrementHonba),
                button(text("-")).on_press_maybe(if self.honba > 0 {
                    Some(Message::DecrementHonba)
                } else {
                    None
                }),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            {
                // Count 5-tiles in hand
                let five_tile_count = self.count_five_tiles();

                if five_tile_count > 0 {
                    row![
                        text(format!("Akadora: {}", self.num_akadora)),
                        button(text("+")).on_press_maybe(
                            if self.num_akadora < five_tile_count && self.num_akadora < 4 {
                                Some(Message::IncrementAkadora)
                            } else {
                                None
                            }
                        ),
                        button(text("-")).on_press_maybe(if self.num_akadora > 0 {
                            Some(Message::DecrementAkadora)
                        } else {
                            None
                        }),
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                } else {
                    row![] // Empty row when no 5-tiles
                }
            },
            column![
                text("Dora Indicators:"),
                row(self
                    .dora_indicators
                    .iter()
                    .map(|t| iced::widget::Image::<iced::widget::image::Handle>::new(
                        get_tile_image_path(t, false)
                    )
                    .width(30)
                    .into())
                    .collect::<Vec<Element<Message>>>())
                .spacing(5),
                button(text("Add")).on_press(Message::StartAddDora),
                if self.is_riichi {
                    column![
                        text("Ura Dora Indicators:"),
                        row(self
                            .uradora_indicators
                            .iter()
                            .map(|t| iced::widget::Image::<iced::widget::image::Handle>::new(
                                get_tile_image_path(t, false)
                            )
                            .width(30)
                            .into())
                            .collect::<Vec<Element<Message>>>())
                        .spacing(5),
                        button(text("Add")).on_press(Message::StartAddUraDora),
                    ]
                    .spacing(5)
                } else {
                    column![]
                }
            ]
            .spacing(5)
        ]
        .spacing(10);

        let calculate_btn = button(text("Calculate Score"))
            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                background_color: Color::from_rgb(0.0, 0.6, 0.0),
                text_color: Color::WHITE,
            })))
            .on_press_maybe(if self.winning_tile.is_some() {
                Some(Message::CalculateScore)
            } else {
                None
            });

        column![
            hand_preview,
            modify_btn,
            winning_tile_section,
            melds_section,
            context_section,
            calculate_btn
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_result(&self) -> Element<'_, Message> {
        use crate::implements::scoring::{AgariResult, HandLimit};
        use crate::implements::yaku::Yaku;

        let content = match &self.score_result {
            Some(Ok(result)) => {
                let AgariResult {
                    han,
                    fu,
                    yaku_list,
                    limit_name,
                    total_payment,
                    oya_payment,
                    ko_payment,
                    honba,
                    agari_type,
                    is_oya,
                    ..
                } = result;

                // Check valid yaku
                let valid_yaku_count = yaku_list
                    .iter()
                    .filter(|y| !matches!(y, Yaku::Dora | Yaku::UraDora | Yaku::AkaDora))
                    .count();

                if valid_yaku_count == 0 && limit_name.is_none() {
                    column![
                        text("No Yaku Found")
                            .size(30)
                            .style(Color::from_rgb(0.8, 0.0, 0.0)),
                        text("You need at least 1 Yaku to win.").size(20),
                        text("(Dora does not count as Yaku)")
                            .size(16)
                            .style(Color::from_rgb(0.5, 0.5, 0.5))
                    ]
                    .spacing(15)
                    .align_items(iced::Alignment::Center)
                } else {
                    // Header
                    let header = text("Calculation Result")
                        .size(30)
                        .style(Color::from_rgb(0.0, 0.0, 0.0));

                    // Total Score
                    let score_text = text(format!("{} Points", total_payment))
                        .size(40)
                        .style(Color::from_rgb(0.8, 0.2, 0.2)); // Accent color

                    // Limit Name (if any)
                    let limit_text = if let Some(limit) = limit_name {
                        let limit_str = match limit {
                            HandLimit::Mangan => "Mangan",
                            HandLimit::Haneman => "Haneman",
                            HandLimit::Baiman => "Baiman",
                            HandLimit::Sanbaiman => "Sanbaiman",
                            HandLimit::Yakuman => "Yakuman",
                        };
                        text(limit_str)
                            .size(24)
                            .style(Color::from_rgb(0.8, 0.0, 0.0))
                    } else {
                        text("")
                    };

                    // Han / Fu
                    let han_fu_text = if limit_name.as_ref() == Some(&HandLimit::Yakuman) {
                        text(format!("{} Han", han)).size(20)
                    } else {
                        text(format!("{} Han / {} Fu", han, fu)).size(20)
                    };

                    // Yaku List
                    let mut yaku_col =
                        column![text("Yaku:").size(18).style(Color::from_rgb(0.3, 0.3, 0.3))];
                    let mut dora_count = 0;
                    let mut uradora_count = 0;
                    let mut akadora_count = 0;

                    for yaku in yaku_list {
                        match yaku {
                            Yaku::Dora => dora_count += 1,
                            Yaku::UraDora => uradora_count += 1,
                            Yaku::AkaDora => akadora_count += 1,
                            _ => {
                                yaku_col = yaku_col.push(text(format!("• {:?}", yaku)).size(18));
                            }
                        }
                    }

                    if dora_count > 0 {
                        yaku_col = yaku_col.push(text(format!("• Dora x{}", dora_count)).size(18));
                    }
                    if uradora_count > 0 {
                        yaku_col =
                            yaku_col.push(text(format!("• Ura Dora x{}", uradora_count)).size(18));
                    }
                    if akadora_count > 0 {
                        yaku_col =
                            yaku_col.push(text(format!("• Aka Dora x{}", akadora_count)).size(18));
                    }

                    // Payment Detail
                    let tsumo_bonus = *honba as u32 * 100;
                    let ron_bonus = *honba as u32 * 300;

                    let payment_text = match (*is_oya, agari_type) {
                        (true, AgariType::Tsumo) => {
                            format!(
                                "Dealer Tsumo\nEach Non-Dealer pays: {} (+{} honba)",
                                oya_payment, tsumo_bonus
                            )
                        }
                        (false, AgariType::Tsumo) => {
                            format!(
                                "Non-Dealer Tsumo\nDealer pays: {} (+{} honba)\nOther Non-Dealers pay: {} (+{} honba)",
                                oya_payment, tsumo_bonus, ko_payment, tsumo_bonus
                            )
                        }
                        (true, AgariType::Ron) => {
                            format!(
                                "Dealer Ron\nDiscarder pays: {} (+{} honba)",
                                total_payment - ron_bonus,
                                ron_bonus
                            )
                        }
                        (false, AgariType::Ron) => {
                            format!(
                                "Non-Dealer Ron\nDiscarder pays: {} (+{} honba)",
                                total_payment - ron_bonus,
                                ron_bonus
                            )
                        }
                    };

                    let payment_section = container(text(payment_text).size(16)).padding(10);

                    column![
                        header,
                        score_text,
                        limit_text,
                        han_fu_text,
                        yaku_col.spacing(5),
                        payment_section
                    ]
                    .spacing(15)
                    .align_items(iced::Alignment::Center)
                }
            }
            Some(Err(_)) => column![
                text("No Yaku Found")
                    .size(30)
                    .style(Color::from_rgb(0.8, 0.0, 0.0)),
                text("You need at least 1 Yaku to win.").size(20),
                text("(Dora does not count as Yaku)")
                    .size(16)
                    .style(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(15)
            .align_items(iced::Alignment::Center),
            None => column![text("No result available.")],
        };

        column![
            content,
            button(text("Start Over")).on_press(Message::StartOver)
        ]
        .spacing(30)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_selecting_winning_tile(&self) -> Element<'_, Message> {
        let mut unique_tiles: Vec<Hai> = self.hand_tiles.iter().map(|t| *t).collect();
        unique_tiles.sort();
        unique_tiles.dedup();

        let tiles: Vec<Element<Message>> = unique_tiles
            .iter()
            .map(|tile| {
                let image_path = get_tile_image_path(tile, false);
                button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(50),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .on_press(Message::SelectWinningTile(*tile))
                .padding(5)
                .into()
            })
            .collect();

        column![
            text("Select Winning Tile").size(24),
            create_grid(tiles, 10),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::ConfirmHand)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_selecting_meld_type(&self) -> Element<'_, Message> {
        column![
            text("Select Meld Type").size(24),
            row![
                button(text("Pon (Koutsu)")).on_press(Message::SelectMeldType(MentsuType::Koutsu)),
                button(text("Chi (Shuntsu)"))
                    .on_press(Message::SelectMeldType(MentsuType::Shuntsu)),
                button(text("Kan (Kantsu)")).on_press(Message::SelectMeldType(MentsuType::Kantsu)),
            ]
            .spacing(20),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::ConfirmHand)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message> {
        let mut valid_tiles = Vec::new();
        let mut counts = [0; 34];
        for t in &self.hand_tiles {
            counts[crate::implements::tiles::tile_to_index(t)] += 1;
        }

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let count = counts[i];
            let is_valid = match m_type {
                MentsuType::Koutsu => count >= 3,
                MentsuType::Kantsu => count >= 4,
                MentsuType::Shuntsu => count > 0,
            };

            if is_valid {
                valid_tiles.push(tile);
            }
        }

        let tiles: Vec<Element<Message>> = valid_tiles
            .iter()
            .map(|tile| {
                let image_path = get_tile_image_path(tile, false);
                button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(50),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .on_press(Message::SelectMeldTile(*tile))
                .padding(5)
                .into()
            })
            .collect();

        column![
            text(format!("Select Tile for {:?}", m_type)).size(24),
            create_grid(tiles, 10),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::ConfirmHand)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_selecting_closed_kan(&self) -> Element<'_, Message> {
        let mut valid_tiles = Vec::new();
        let mut counts = [0; 34];
        for t in &self.hand_tiles {
            counts[crate::implements::tiles::tile_to_index(t)] += 1;
        }

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            if counts[i] == 4 {
                valid_tiles.push(tile);
            }
        }

        let tiles: Vec<Element<Message>> = valid_tiles
            .iter()
            .map(|tile| {
                let image_path = get_tile_image_path(tile, false);
                button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(50),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .on_press(Message::SelectClosedKan(*tile))
                .padding(5)
                .into()
            })
            .collect();

        column![
            text("Select Tile for Closed Kan").size(24),
            create_grid(tiles, 10),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::ConfirmHand)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message> {
        let mut tiles = Vec::new();

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let image_path = get_tile_image_path(&tile, false);

            let btn = button(
                iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(40),
            )
            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                background_color: Color::WHITE,
                text_color: Color::BLACK,
            })))
            .on_press(if is_ura {
                Message::SelectUraDora(tile)
            } else {
                Message::SelectDora(tile)
            })
            .padding(5)
            .into();

            tiles.push(btn);
        }

        column![
            text("Select").size(24),
            create_grid(tiles, 9),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::ConfirmHand)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_hand_preview(&self) -> Element<'_, Message> {
        let tiles: Vec<Element<Message>> = self
            .hand_tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let image_path = get_tile_image_path(tile, false);
                button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(40),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .on_press(Message::RemoveTile(i))
                .padding(0)
                .into()
            })
            .collect();

        container(row(tiles).spacing(5))
            .height(Length::Fixed(80.0))
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    fn view_hand_preview_locked(&self) -> Element<'_, Message> {
        let tiles: Vec<Element<Message>> = self
            .hand_tiles
            .iter()
            .enumerate()
            .map(|(_, tile)| {
                let image_path = get_tile_image_path(tile, false);

                let btn = button(
                    iced::widget::Image::<iced::widget::image::Handle>::new(image_path).width(40),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .padding(0);

                btn.into()
            })
            .collect();

        row(tiles).spacing(5).into()
    }

    fn view_tile_pool(&self) -> Element<'_, Message> {
        let mut tiles = Vec::new();

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let count = self.tile_counts[i];
            let image_path = get_tile_image_path(&tile, false);

            let tile_image = image(image_path).width(50);

            let count_text = text(format!("({})", count)).size(12).style(if count > 0 {
                Color::BLACK
            } else {
                Color::from_rgb(0.5, 0.5, 0.5)
            });

            // Use different button background color to create dimming effect when tiles run out
            let button_bg_color = if count > 0 {
                Color::WHITE
            } else {
                Color::from_rgb(0.85, 0.85, 0.85) // Light grey for dimmed appearance
            };

            let btn = button(column![tile_image, count_text].align_items(iced::Alignment::Center))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: button_bg_color,
                    text_color: Color::BLACK,
                })))
                .on_press_maybe(if count > 0 {
                    Some(Message::AddTile(tile))
                } else {
                    None
                })
                .padding(5)
                .into();

            tiles.push(btn);
        }

        create_grid(tiles, 9)
    }

    fn get_meld_tiles(&self, meld: &OpenMeldInput) -> Vec<Hai> {
        let mut tiles = Vec::new();
        match meld.mentsu_type {
            MentsuType::Shuntsu => {
                let start_idx = crate::implements::tiles::tile_to_index(&meld.representative_tile);
                if start_idx < 27 {
                    let suit_base = (start_idx / 9) * 9;

                    for i in 0..3 {
                        let idx = start_idx + i;
                        if idx < suit_base + 9 {
                            tiles.push(crate::implements::tiles::index_to_tile(idx));
                        }
                    }
                } else {
                    tiles.push(meld.representative_tile);
                }
            }
            MentsuType::Koutsu => {
                for _ in 0..3 {
                    tiles.push(meld.representative_tile);
                }
            }
            MentsuType::Kantsu => {
                for _ in 0..4 {
                    tiles.push(meld.representative_tile);
                }
            }
        }
        tiles
    }

    /// Counts the total number of 5-tiles in the hand (including winning tile, melds, and kans)
    fn count_five_tiles(&self) -> u8 {
        let mut count = 0;

        // Count 5-tiles in hand_tiles
        for tile in &self.hand_tiles {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 1;
            }
        }

        // Count 5-tiles in winning_tile
        if let Some(tile) = &self.winning_tile {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 1;
            }
        }

        // Count 5-tiles in open_melds
        for meld in &self.open_melds {
            let tiles = self.get_meld_tiles(meld);
            for tile in tiles {
                if matches!(tile, Hai::Suhai(5, _)) {
                    count += 1;
                }
            }
        }

        // Count 5-tiles in closed_kans
        for tile in &self.closed_kans {
            if matches!(tile, Hai::Suhai(5, _)) {
                // Closed kan has 4 tiles
                count += 4;
            }
        }

        count
    }
}

#[allow(dead_code)]
trait OnPressMaybe {
    fn on_press_maybe(self, msg: Option<Message>) -> Self;
}

impl<'a> OnPressMaybe for button::Button<'a, Message> {
    fn on_press_maybe(self, msg: Option<Message>) -> Self {
        match msg {
            Some(m) => self.on_press(m),
            None => self,
        }
    }
}

fn get_tile_image_path(tile: &Hai, is_akadora: bool) -> String {
    if is_akadora {
        match tile {
            Hai::Suhai(5, Suhai::Manzu) => return "lib/Man5-Dora.png".to_string(),
            Hai::Suhai(5, Suhai::Pinzu) => return "lib/Pin5-Dora.png".to_string(),
            Hai::Suhai(5, Suhai::Souzu) => return "lib/Sou5-Dora.png".to_string(),
            _ => {}
        }
    }

    let filename = match tile {
        Hai::Suhai(n, Suhai::Manzu) => format!("Man{}.png", n),
        Hai::Suhai(n, Suhai::Pinzu) => format!("Pin{}.png", n),
        Hai::Suhai(n, Suhai::Souzu) => format!("Sou{}.png", n),
        Hai::Jihai(Jihai::Kaze(Kaze::Ton)) => "Ton.png".to_string(),
        Hai::Jihai(Jihai::Kaze(Kaze::Nan)) => "Nan.png".to_string(),
        Hai::Jihai(Jihai::Kaze(Kaze::Shaa)) => "Shaa.png".to_string(),
        Hai::Jihai(Jihai::Kaze(Kaze::Pei)) => "Pei.png".to_string(),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Haku)) => "Haku.png".to_string(),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu)) => "Hatsu.png".to_string(),
        Hai::Jihai(Jihai::Sangen(Sangenpai::Chun)) => "Chun.png".to_string(),
    };
    format!("lib/{}", filename)
}

fn create_grid(elements: Vec<Element<Message>>, columns: usize) -> Element<Message> {
    let mut rows = column![].spacing(10);
    let mut current_row = row![].spacing(10);
    let mut count_in_row = 0;

    for element in elements {
        current_row = current_row.push(element);
        count_in_row += 1;

        if count_in_row >= columns {
            rows = rows.push(current_row);
            current_row = row![].spacing(10);
            count_in_row = 0;
        }
    }

    if count_in_row > 0 {
        rows = rows.push(current_row);
    }

    rows.into()
}

struct ColoredButtonStyle {
    background_color: Color,
    text_color: Color,
}

impl button::StyleSheet for ColoredButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.background_color)),
            text_color: self.text_color,
            border: iced::Border::with_radius(4.0),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(iced::Background::Color(Color {
                a: 0.8,
                ..self.background_color
            })),
            ..active
        }
    }
}
