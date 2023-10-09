import type {Href, Logo} from "@src/types/common.ts";

export interface Link {
    name: string;
    logo: Logo | null;
    href: Href;
    hide: boolean | null;
    description: string | null;
    blank: boolean | null;
    target: string | null;
}
