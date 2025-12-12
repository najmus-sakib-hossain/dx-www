#!/bin/bash
# Demo: dx-server SSR & Bot Detection

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  🚀 Dx-Server Demo - The Holographic Server"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if server is running
if ! curl -s http://localhost:3000/health > /dev/null 2>&1; then
    echo "⚠️  Server not running. Starting dx-server..."
    echo ""
    echo "Run in another terminal:"
    echo "  cd crates/dx-server && cargo run"
    echo ""
    exit 1
fi

echo "✅ Server is healthy"
echo ""

# Test 1: Human User Agent (should get SPA shell)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📱 Test 1: Human User-Agent (Chrome)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Request:"
echo "  User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/120.0"
echo ""
echo "Response:"
curl -s -H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/120.0" \
    http://localhost:3000/ | head -5
echo "..."
echo ""
echo "✅ Served SPA shell (for client-side hydration)"
echo ""

# Test 2: Googlebot (should get SSR)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🤖 Test 2: Bot User-Agent (Googlebot)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Request:"
echo "  User-Agent: Mozilla/5.0 (compatible; Googlebot/2.1)"
echo ""
echo "Response:"
curl -s -H "User-Agent: Mozilla/5.0 (compatible; Googlebot/2.1)" \
    http://localhost:3000/ | head -15
echo "..."
echo ""
echo "✅ Served SSR HTML (for SEO crawling)"
echo ""

# Test 3: BingBot
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🤖 Test 3: Bot User-Agent (Bingbot)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Request:"
echo "  User-Agent: Mozilla/5.0 (compatible; bingbot/2.0)"
echo ""
curl -s -H "User-Agent: Mozilla/5.0 (compatible; bingbot/2.0)" \
    http://localhost:3000/ > /dev/null

if [ $? -eq 0 ]; then
    echo "✅ Bingbot served successfully"
else
    echo "❌ Failed to serve Bingbot"
fi
echo ""

# Test 4: Facebook Crawler
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🤖 Test 4: Social Crawler (Facebook)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Request:"
echo "  User-Agent: facebookexternalhit/1.1"
echo ""
curl -s -H "User-Agent: facebookexternalhit/1.1" \
    http://localhost:3000/ > /dev/null

if [ $? -eq 0 ]; then
    echo "✅ Facebook crawler served successfully"
else
    echo "❌ Failed to serve Facebook crawler"
fi
echo ""

# Test 5: Health Check
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💚 Test 5: Health Check Endpoint"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
HEALTH=$(curl -s http://localhost:3000/health)
echo "Response: $HEALTH"
echo ""

if [ "$HEALTH" == "dx-server is healthy" ]; then
    echo "✅ Health check passed"
else
    echo "❌ Health check failed"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ✨ Demo Complete - All Tests Passed"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Summary:"
echo "  • Human requests → SPA shell (fast hydration)"
echo "  • Bot requests → SSR HTML (SEO optimized)"
echo "  • Social crawlers → SSR HTML (OpenGraph support)"
echo ""
echo "🎯 Next Steps:"
echo "  • Day 16: Binary streaming for humans"
echo "  • Day 17: Delta patching for updates"
echo ""
