// Test just the module content
(function(exports, require, module){
    // Simple JavaScript test 
    function hello() {
        console.log('Hello from module 0');
    }
    
    export default hello;
});
