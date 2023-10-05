export interface Message {
    type: string;
    message: string;
}

export interface Group {
    name: string;
    items: Item[];
}

export interface Item {
    name: string;
    url: string;
    description: string | null;
    logo: Logo | null;
}

export interface Logo {
    Emj: string | undefined;
    Img: string | undefined;
}
