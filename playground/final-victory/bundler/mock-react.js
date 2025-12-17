// Mock React for bundler test
const React = {
  createElement: function(type, props, ...children) {
    return { type, props: props || {}, children };
  },
  
  useState: function(initial) {
    let state = initial;
    const setState = (newValue) => {
      state = typeof newValue === 'function' ? newValue(state) : newValue;
    };
    return [state, setState];
  },
  
  useEffect: function(fn, deps) {
    fn();
  },
  
  useCallback: function(fn, deps) {
    return fn;
  },
  
  useMemo: function(fn, deps) {
    return fn();
  },
  
  Component: class Component {
    constructor(props) {
      this.props = props;
    }
  }
};

export default React;
