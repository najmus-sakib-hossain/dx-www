// Example: Complex Dashboard App
// This will be compiled to Macro runtime (7.5 KB)

function Dashboard() {
    const [users, setUsers] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [selectedUser, setSelectedUser] = useState(null);
    const [searchQuery, setSearchQuery] = useState("");
    const [sortOrder, setSortOrder] = useState("asc");
    const [filterStatus, setFilterStatus] = useState("all");
    
    useEffect(() => {
        async function fetchUsers() {
            try {
                const response = await fetch("/api/users");
                const data = await response.json();
                setUsers(data);
                setLoading(false);
            } catch (err) {
                setError(err.message);
                setLoading(false);
            }
        }
        fetchUsers();
    }, []);
    
    const filteredUsers = users
        .filter(u => u.name.includes(searchQuery))
        .filter(u => filterStatus === "all" || u.status === filterStatus)
        .sort((a, b) => sortOrder === "asc" ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
    
    return (
        <div className="dashboard">
            <Header />
            <Sidebar />
            <main>
                <SearchBar 
                    value={searchQuery} 
                    onChange={setSearchQuery}
                />
                <FilterBar 
                    status={filterStatus}
                    onStatusChange={setFilterStatus}
                    sortOrder={sortOrder}
                    onSortChange={setSortOrder}
                />
                {loading && <LoadingSpinner />}
                {error && <ErrorBanner message={error} />}
                <UserTable 
                    users={filteredUsers}
                    onSelect={setSelectedUser}
                    selected={selectedUser}
                />
                {selectedUser && (
                    <UserDetail 
                        user={selectedUser}
                        onClose={() => setSelectedUser(null)}
                    />
                )}
            </main>
            <Footer />
        </div>
    );
}

function Header() { /* ... */ }
function Sidebar() { /* ... */ }
function SearchBar({ value, onChange }) { /* ... */ }
function FilterBar({ status, onStatusChange, sortOrder, onSortChange }) { /* ... */ }
function LoadingSpinner() { /* ... */ }
function ErrorBanner({ message }) { /* ... */ }
function UserTable({ users, onSelect, selected }) { /* ... */ }
function UserDetail({ user, onClose }) { /* ... */ }
function Footer() { /* ... */ }

export default Dashboard;
