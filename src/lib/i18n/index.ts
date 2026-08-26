import { addMessages, init, locale, getLocaleFromNavigator } from "svelte-i18n";
import { store } from "$lib/store.svelte";

export type Lang = "en" | "ru";

export const en = {
  app: {
    name: "MetaGrid"
  },
  status: {
    fresh: "Up to date",
    stale: "Stale",
    error: "Error",
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
    other: "OTHER {role} HEROES - ORDERED BY D2PT RATING (AND PICKRATE)"
  },
  settings: {
    title: "Settings",
    role_labels: "Role labels",
    role_named: "Carry, Mid...",
    role_pos: "POS 1, POS 2...",
    role_named_subtext: "Carry, Mid, Offlane...",
    role_pos_subtext: "POS 1, POS 2, POS 3...",
    interval: "Refresh interval",
    interval_hours: "{n} h",
    interval_30m: "30 m",
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
    fetched: "Meta loaded",
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
    other: "ДРУГИЕ {role} - ПО D2PT РЕЙТИНГУ (И ПИКРЕЙТУ)"
  },
  settings: {
    title: "Настройки",
    role_labels: "Подписи ролей",
    role_named: "Керри, Мид...",
    role_pos: "ПОЗ 1, ПОЗ 2...",
    role_named_subtext: "Керри, Мид, Оффлейн...",
    role_pos_subtext: "ПОЗ 1, ПОЗ 2, ПОЗ 3...",
    interval: "Интервал обновления",
    interval_hours: "{n} ч",
    interval_30m: "30 мин",
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
    fetched: "Мета загружена",
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
  }
};

addMessages("en", en);
addMessages("ru", ru);

export function getInitialLang(): Lang {
  return "en";
}

export function setupI18n(initialLocale: Lang = getInitialLang()) {
  init({
    fallbackLocale: "en",
    initialLocale,
  });
}

export async function setLanguage(l: Lang) {
  locale.set(l);
  if (store.settings) {
    await store.saveSettings({ lang: l });
  }
}
