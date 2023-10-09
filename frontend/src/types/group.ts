import type {Service} from "@src/types/service.ts";

export interface Group {
    name: string;
    items: Service[];
}
