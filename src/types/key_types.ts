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
    { value: 'qwerty', label: 'QWERTY' },
    { value: 'azerty', label: 'AZERTY' },
];

export const languageOptions = [
    { value: 'english', label: 'English' },
    { value: 'french', label: 'French' },
];

export const themeOptions = [
    { value: 'light', label: 'Light' },
    { value: 'dark', label: 'Dark' },
];

export const difficultyOptions = [
    { value: 'beginner', label: 'Beginner' },
    { value: 'intermediate', label: 'Intermediate' },
    { value: 'advanced', label: 'Advanced' },
];