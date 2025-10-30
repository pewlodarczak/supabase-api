// k6 run script.js --env URL=https://supabase-api-8vc9.shuttle.app/users
import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
    scenarios: {
        infinite: {
            executor: 'externally-controlled',
            vus: 50,          // number of VUs (simulated users)
            duration: '0s',   // 0s -> infinite for this executor
        },
    },
};

export default function () {
    http.get(__ENV.URL || 'https://supabase-api-8vc9.shuttle.app/');
    sleep(1);
}
