export function useState<T>(initial: T): [T, (value: T) => void] { return [initial, () => {}]; }
