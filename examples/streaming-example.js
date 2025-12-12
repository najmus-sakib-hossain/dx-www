/**
 * dx-www Streaming Example
 * 
 * Demonstrates how to use the streaming API to progressively load
 * and render a dx-www application.
 */

import init, {
    init_streaming,
    feed_chunk_data,
    poll_and_process_chunk,
    is_stream_finished,
    finalize_stream
} from './pkg/dx_client.js';

/**
 * High-level wrapper for processing a ReadableStream
 * 
 * @param {ReadableStream} stream - The stream to process
 * @returns {Promise<void>}
 */
async function processStream(stream) {
    const reader = stream.getReader();
    
    try {
        while (true) {
            const { done, value } = await reader.read();
            
            if (done) {
                console.log('Stream complete');
                break;
            }
            
            // Feed chunk to WASM
            const chunksReady = feed_chunk_data(value);
            console.log(`Received ${value.length} bytes, ${chunksReady} chunks ready`);
            
            // Process all ready chunks
            for (let i = 0; i < chunksReady; i++) {
                const hasMore = poll_and_process_chunk();
                if (!hasMore) break;
            }
        }
        
        // Finalize if stream finished
        if (is_stream_finished()) {
            console.log('Finalizing stream...');
            finalize_stream();
        }
    } finally {
        reader.releaseLock();
    }
}

/**
 * Main application entry point
 */
async function main() {
    console.log('Initializing dx-www runtime...');
    
    // Initialize WASM module
    await init();
    
    // Initialize streaming components
    init_streaming();
    console.log('Streaming initialized');
    
    // Fetch and stream the application
    console.log('Fetching /app.dxb...');
    const response = await fetch('/app.dxb');
    
    if (!response.ok) {
        throw new Error(`Failed to fetch: ${response.statusText}`);
    }
    
    if (!response.body) {
        throw new Error('ReadableStream not supported');
    }
    
    // Process the stream
    await processStream(response.body);
    
    console.log('Application loaded!');
}

// Run on page load
main().catch(error => {
    console.error('Error:', error);
});
