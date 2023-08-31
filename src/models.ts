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
                kind: FieldKind.Unk64,
                offset: i * 8
            })
        }

        this.classes = [{
            name: 'NewClass',
            uuid: crypto.randomUUID(),
            properties
        }]
    }

    newClass(name: string): Class {
        const uuid = crypto.randomUUID()
        const item: Class = {
            properties: [],
            uuid, name,
        };
        this.classes.push(item)
        return item;
    }

    getClass(uuid: string): Class | undefined {
        return this.classes.find(c => c.uuid == uuid);
    }
}

export interface Class {
    name: string
    uuid: string
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

