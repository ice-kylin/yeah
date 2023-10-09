import type {Href} from "@src/types/common.ts";

export function getHref(href: Href) {
    const url = href.Url;
    if (url !== undefined) {
        return url;
    }

    const rules = href.Rules;
    if (rules !== undefined) {
        for (const key in rules) {
            if (rules.hasOwnProperty(key)) {
                const value = rules[key];
                let reg = new RegExp(key);

                if (reg.test(location.host)) {
                    return value.replaceAll("${host}", location.hostname);
                }
            }
        }
    }
}
