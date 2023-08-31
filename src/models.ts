import { writable, type Writable } from "svelte/store";

export interface ProcessEntry {
    name: string;
    id: number;
}

export enum FieldKind {
    Unk8, Unk16, Unk32, Unk64,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    Ptr, StrPtr
}

export class ProjectData {
    classes: Class[] = []

    constructor() {
        let properties: Property[] = []
        for (let i = 0; i < 10; i++) {
            properties.push({
                kind: FieldKind.Unk16,
                offset: i * 2
            })
        }

        this.classes = [{
            name: 'NewClass',
            uuid: crypto.randomUUID(),
            address: 0,
            properties
        }]
    }

    newClass(name: string): Class {
        const uuid = crypto.randomUUID()
        const item: Class = {
            properties: [],
            uuid, name,
            address: 0
        };
        this.classes.push(item)
        return item;
    }

    getClass(uuid: string): Class {
        const item = this.classes.find(c => c.uuid == uuid);
        if (!item) throw 'unreachable';

        return item;
    }
}

export interface Class {
    name: string
    uuid: string
    address: number
    properties: Property[]
}

export interface Property {
    name?: string
    kind: FieldKind
    offset: number
    data?: any
}

export let project_data: Writable<ProjectData> = writable(new ProjectData());
export let attached: Writable<boolean> = writable(false);

