export type KeyboardKeyType =
    | { type: 'Number', name: string }
    | { type: 'Letter', name: string }
    | { type: 'Special', name: string };


export function createKeyType(key: string): KeyboardKeyType {
    if (!isNaN(Number(key))) {
        return { type: 'Number', name: key };
    } else if (/^[a-zA-Z]$/.test(key)) {
        return { type: 'Letter', name: key };
    } else {
        return { type: 'Special', name: key };
    }
}

export function handleKeyPress(key: string): string {
    const keyType: KeyboardKeyType = createKeyType(key);
    console.log(`${keyType.type} key pressed:`, keyType.name);
    return keyType.name;
}

export const keyboardOptions = [
    { value: 'QWERTY', label: 'QWERTY' },
    { value: 'AZERTY', label: 'AZERTY' },
];

export const languageOptions = [
    { value: 'ENGLISH', label: 'English' },
    { value: 'FRENCH', label: 'French' },
];

export const themeOptions = [
    { value: 'LIGHT', label: 'Light' },
    { value: 'DARK', label: 'Dark' },
];

export const difficultyOptions = [
    { value: 'BEGINNER', label: 'Beginner' },
    { value: 'INTERMEDIATE', label: 'Intermediate' },
    { value: 'ADVANCED', label: 'Advanced' },
];