// https://github.com/karatelabs/karate?tab=readme-ov-file#karate-configjs
function fn() {
    var env = karate.env;
    karate.log('karate.env system property was:', env);
    if (!env) {
        env = 'dev';
    }

    var config = {
        baseUrl: 'http://host.docker.internal:55432',
    };

    // don't waste time waiting for a connection or if servers don't respond within 5 seconds
    karate.configure('connectTimeout', 5000);
    karate.configure('readTimeout', 5000);
    return config;
}
