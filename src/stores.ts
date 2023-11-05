import { writable } from 'svelte/store';


//export const encrypted = writable({});
export type accountType = {
    title: string,
    site: string,
    email: string,
    username: string,
    password: string,
    notes: string,
    humanname: string,
    yearOfBirth: number | string,
    monthOfBirth: number | string,
    dayOfBirth: number | string,
    
}
export const defaultDecrypted = {
    accounts: [] as accountType[]
} 

export const startup_data      = writable({});
export const stored_key        = writable("");
export const decrypted         = writable(defaultDecrypted);
export let futureIndex = -1;
    decrypted.subscribe(value => {
        futureIndex = value.accounts.length
    })
/**
 * @description -2 is no account - needs master, -1 is no account, 0+ is index.
 */
export const selected_account  = writable(-2) 