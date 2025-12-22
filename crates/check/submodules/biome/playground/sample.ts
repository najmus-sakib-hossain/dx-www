// Sample TypeScript file to test Biome formatter and linter
// This file demonstrates TypeScript-specific features

interface User {
    id: number;
    name: string;
    email: string;
    roles?: string[];
}

type Status = "active" | "inactive" | "pending";

class UserService {
    private users: Map<number, User>;

    constructor() {
        this.users = new Map();
    }

    addUser(user: User): void {
        this.users.set(user.id, user);
    }

    getUser(id: number): User | undefined {
        return this.users.get(id);
    }

    updateStatus(id: number, status: Status): boolean {
        const user = this.users.get(id);
        if (user) {
            // Update logic here
            return true;
        }
        return false;
    }
}

// Generic function
function identity<T>(arg: T): T {
    return arg;
}

// Enum
enum Color {
    Red = "RED",
    Green = "GREEN",
    Blue = "BLUE",
}

// Tuple
type Point = [number, number];
const origin: Point = [0, 0];

// Utility types
type Partial<T> = {
    [P in keyof T]?: T[P];
};

export { UserService, Color, identity, type User, type Status };
