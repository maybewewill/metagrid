export const en = {
  app: {
    name: "MetaGrid"
  },
  status: {
    fresh: "Up to date",
    stale: "Stale",
    error: "Error",
    fetching: "Fetching...",
    refreshing: "Fetching & Patching...",
    idle: "Idle",
    updated: "Updated {time}",
    fetch: "Fetch & Patch",
    refresh: "Fetch & Patch",
    list: "List",
    grid: "Grid"
  },
  role: {
    pos1: "POS 1 — Carry",
    pos2: "POS 2 — Mid",
    pos3: "POS 3 — Offlane",
    pos4: "POS 4 — Support",
    pos5: "POS 5 — Hard Support"
  },
  role_short: {
    pos1: "Carry",
    pos2: "Mid",
    pos3: "Offlane",
    pos4: "Support",
    pos5: "Hard Support"
  },
  role_upper: {
    pos1: "CARRY",
    pos2: "MID",
    pos3: "OFFLANE",
    pos4: "SUPPORT",
    pos5: "HARD SUPPORT"
  },
  pos_prefix: "POS",
  grid: {
    top: "TOP {role} HEROES - ORDERED BY D2PT ELO",
    other: "OTHER {role} HEROES - ORDERED BY MATCHES (AND PICKRATE)"
  },
  settings: {
    title: "Settings",
    role_labels: "Role labels",
    role_named: "Carry, Mid...",
    role_pos: "POS 1, POS 2...",
    role_named_subtext: "Carry, Mid, Offlane...",
    role_pos_subtext: "POS 1, POS 2, POS 3...",
    meta_source: "Data source",
    meta_source_pubs: "Pubs",
    meta_source_tournaments: "Tournaments",
    meta_source_pubs_hint: "High-MMR public match tracking (8k+ MMR)",
    meta_source_tournaments_hint: "Official competitive tournament matches",
    grid_mode: "Grid Mode",
    grid_mode_separate: "Separate",
    grid_mode_merge: "Merged",
    grid_mode_separate_hint: "Creates 5 separate role grids",
    grid_mode_merge_hint: "Injects META block into an existing grid",
    merge_target: "Target Grid",
    merge_target_hint: "META block will be injected into this grid, on the left",
    merge_target_placeholder: "Select grid…",
    merge_target_empty: "No custom grids found",
    interval: "Refresh interval",
    interval_hours: "{n} h",
    interval_30m: "30 min",
    interval_15m: "15 min",
    account: "Steam Account",
    all_accounts: "All accounts",
    autostart: "Start with Windows",
    autostart_hint: "Launch MetaGrid on login",
    language: "Language",
    updates: "Updates",
    check_updates: "Check for updates",
    checking_updates: "Checking...",
    up_to_date: "You have the latest version",
    update_available: "Version {version} is available",
    download_update: "Download update",
    save: "Save",
    saved: "Settings saved",
    back: "Back"
  },
  onboarding: {
    welcome: "Welcome to MetaGrid",
    subtitle: "A fresh meta hero grid, kept up to date in the background.",
    pick_account: "Write the grid to",
    fetch: "Fetch & Patch",
    fetch_meta: "Fetch Meta",
    fetched: "Meta loaded",
    choose_mode: "Choose Grid Mode",
    choose_mode_desc: "Select how MetaGrid should arrange your hero layouts in Dota 2:",
    tip_settings: "You can change this at any time in Settings.",
    finish: "Get started",
    no_accounts: "No Dota accounts found — launch Dota once, then reopen MetaGrid."
  },
  toast: {
    updated: "Meta updated",
    error: "Refresh failed"
  },
  empty: {
    title: "No meta yet",
    desc: "Pull the current pro meta and build your hero grid.",
    cta: "Fetch & Patch"
  },
  tournaments: {
    select: "Tournament",
    search: "Search tournaments...",
    all: "All Tournaments",
    matches: "{count} matches",
    no_results: "No tournaments found"
  },
  update_modal: {
    title: "New Update Available",
    subtitle: "A new version of MetaGrid is ready to install.",
    current_vs_latest: "v{current} → {latest}",
    whats_new: "What's New:",
    update_now: "Update Now",
    later: "Later",
    downloading: "Downloading update...",
    launching: "Launching installer...",
    failed: "Update failed: {error}",
    retry: "Retry"
  }
};

export const ru = {
  app: {
    name: "MetaGrid"
  },
  status: {
    fresh: "Актуально",
    stale: "Устарело",
    error: "Ошибка",
    fetching: "Загрузка...",
    refreshing: "Fetching & Patching...",
    idle: "Ожидание",
    updated: "Обновлено {time}",
    fetch: "Fetch & Patch",
    refresh: "Fetch & Patch",
    list: "Список",
    grid: "Сетка"
  },
  role: {
    pos1: "ПОЗ 1 — Керри",
    pos2: "ПОЗ 2 — Мид",
    pos3: "ПОЗ 3 — Оффлейн",
    pos4: "ПОЗ 4 — Саппорт",
    pos5: "ПОЗ 5 — Хард Саппорт"
  },
  role_short: {
    pos1: "Керри",
    pos2: "Мид",
    pos3: "Оффлейн",
    pos4: "Саппорт",
    pos5: "Хард Саппорт"
  },
  role_upper: {
    pos1: "КЕРРИ",
    pos2: "МИД",
    pos3: "ОФФЛЕЙН",
    pos4: "САППОРТ",
    pos5: "ХАРД САППОРТ"
  },
  pos_prefix: "ПОЗ",
  grid: {
    top: "ТОП {role} - ПО D2PT ELO",
    other: "ДРУГИЕ {role} - ПО МАТЧАМ (И ПИКРЕЙТУ)"
  },
  settings: {
    title: "Настройки",
    role_labels: "Подписи ролей",
    role_named: "Керри, Мид...",
    role_pos: "ПОЗ 1, ПОЗ 2...",
    role_named_subtext: "Керри, Мид, Оффлейн...",
    role_pos_subtext: "ПОЗ 1, ПОЗ 2, ПОЗ 3...",
    meta_source: "Источник данных",
    meta_source_pubs: "Паблики",
    meta_source_tournaments: "Турниры",
    meta_source_pubs_hint: "Паблики про-игроков (8k+ MMR)",
    meta_source_tournaments_hint: "Официальные профессиональные турниры",
    grid_mode: "Режим грида",
    grid_mode_separate: "Отдельно",
    grid_mode_merge: "В существующий",
    grid_mode_separate_hint: "Создавать 5 отдельных грид-конфигов",
    grid_mode_merge_hint: "Вставлять META-блок в существующий грид",
    merge_target: "Целевой грид",
    merge_target_hint: "META-блок добавляется в этот грид, слева",
    merge_target_placeholder: "Выбери грид…",
    merge_target_empty: "Кастомных гридов не найдено",
    interval: "Интервал обновления",
    interval_hours: "{n} ч",
    interval_30m: "30 мин",
    interval_15m: "15 мин",
    account: "Аккаунт Steam",
    all_accounts: "Все аккаунты",
    autostart: "Запуск с Windows",
    autostart_hint: "Запускать MetaGrid при входе",
    language: "Язык",
    updates: "Обновления",
    check_updates: "Проверить обновления",
    checking_updates: "Проверка...",
    up_to_date: "У вас установлена последняя версия",
    update_available: "Доступна версия {version}",
    download_update: "Скачать обновление",
    save: "Сохранить",
    saved: "Настройки сохранены",
    back: "Назад"
  },
  onboarding: {
    welcome: "Добро пожаловать в MetaGrid",
    subtitle: "Актуальная мета-сетка героев, обновляется в фоне.",
    pick_account: "Куда записывать сетку",
    fetch: "Fetch & Patch",
    fetch_meta: "Загрузить мету",
    fetched: "Мета загружена",
    choose_mode: "Выберите режим сетки",
    choose_mode_desc: "Выберите, как MetaGrid должен разместить сетки героев в Dota 2:",
    tip_settings: "В любое время можно изменить в настройках.",
    finish: "Начать",
    no_accounts: "Аккаунты Dota не найдены — запустите Dota один раз и откройте MetaGrid снова."
  },
  toast: {
    updated: "Мета обновлена",
    error: "Не удалось обновить"
  },
  empty: {
    title: "Меты пока нет",
    desc: "Забрать актуальную про-мету и собрать сетку героев.",
    cta: "Fetch & Patch"
  },
  tournaments: {
    select: "Турнир",
    search: "Поиск турнира...",
    all: "Все турниры",
    matches: "{count} матчей",
    no_results: "Турниры не найдены"
  },
  update_modal: {
    title: "Доступно обновление",
    subtitle: "Новая версия MetaGrid готова к установке.",
    current_vs_latest: "v{current} → {latest}",
    whats_new: "Что нового:",
    update_now: "Обновить сейчас",
    later: "Позже",
    downloading: "Скачивание обновления...",
    launching: "Запуск установщика...",
    failed: "Ошибка обновления: {error}",
    retry: "Повторить"
  }
};
