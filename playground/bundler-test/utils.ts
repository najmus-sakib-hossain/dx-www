// Utility functions
export function useState<T>(initial: T): [T, (value: T) => void] {
  let state = initial;
  
  const setState = (newValue: T) => {
    state = newValue;
    render();
  };
  
  return [state, setState];
}

export function useEffect(fn: () => void, deps: any[]) {
  // Effect implementation
  fn();
}

export const helpers = {
  format: (value: number): string => `Count: ${value}`,
  parse: (str: string): number => parseInt(str, 10),
};
