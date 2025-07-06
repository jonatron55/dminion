import { writable } from "svelte/store";

export type MessageBoxButton = {
    label: string;
    action?: () => void;
};

export interface MessageBox {
    title: string;
    content: string;
    severity: "" | "ok" | "caution" | "danger";
    affirmativeButton: MessageBoxButton;
    negativeButton?: MessageBoxButton;
    cancelButton?: MessageBoxButton;
}


function createMessageBoxStore() {
    const { subscribe, set } = writable<MessageBox | undefined>(undefined);
    return {
        subscribe,
        show: (msg: MessageBox) => set(msg),
        clear: () => set(undefined)
    };
}

export const messageBoxStore = createMessageBoxStore();
