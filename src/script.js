// k6 load testing script
// k6 run script:src/script.js --vus 50 --duration 1m

import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
    vus: 50,           // number of virtual users
    //duration: '1m',    // test duration
    duration: '3650h', // ~ 5 months — effectively "forever" for short-term purposes
};

export default function () {
    // 👇 Replace this URL with your target
    let res = http.get('https://supabase-api-8vc9.shuttle.app/');

    // Wait 1 second before the next request (simulates user “think time”)
    sleep(1);
}
