// Component example with TSX
interface ComponentProps {
  count: number;
  onClick: () => void;
}

export function Component({ count, onClick }: ComponentProps) {
  return (
    <div className="component">
      <span>Current count: {count}</span>
      <button onClick={onClick}>Click me</button>
    </div>
  );
}

// Unused component for tree-shaking test
export function UnusedComponent() {
  return <div>This should be removed!</div>;
}
