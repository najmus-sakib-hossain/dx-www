import React, { useState } from 'react';
import UserCard from './components/UserCard';

function App() {
  const [users, setUsers] = useState([
    { id: 1, name: 'Alice', email: 'alice@example.com' },
    { id: 2, name: 'Bob', email: 'bob@example.com' }
  ]);

  return (
    <div className="app">
      <h1>User List</h1>
      {users.map(user => (
        <UserCard key={user.id} user={user} />
      ))}
    </div>
  );
}

export default App;
