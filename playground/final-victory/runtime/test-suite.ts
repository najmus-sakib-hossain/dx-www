// TypeScript Test Suite
interface User {
  id: number;
  name: string;
  email: string;
  age: number;
}

class UserService {
  private users: User[] = [];

  addUser(user: User): void {
    this.users.push(user);
  }

  getActiveUsers(): User[] {
    return this.users.filter(u => u.age >= 18);
  }

  getTotalAge(): number {
    return this.users.reduce((sum, u) => sum + u.age, 0);
  }
}

// Create test data
const service = new UserService();
for (let i = 0; i < 10000; i++) {
  service.addUser({
    id: i,
    name: `User ${i}`,
    email: `user${i}@test.com`,
    age: 15 + (i % 50)
  });
}

console.log('Active users:', service.getActiveUsers().length);
console.log('Total age:', service.getTotalAge());
