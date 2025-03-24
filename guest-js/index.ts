import { invoke } from "@tauri-apps/api/core";

export const COMMAND = {
  GET_LOCALE: "plugin:locale|get_locale",
};

/**
 * Get the locale of the system.
 *
 * @example
 * ```
 * import { getLocale } from 'tauri-plugin-locale-api';
 *
 * const locale = await getLocale();
 * console.log(locale); // en-US
 * ```
 */
export const getLocale = () => {
  return invoke<string>(COMMAND.GET_LOCALE);
};
