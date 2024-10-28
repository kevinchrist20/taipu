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
