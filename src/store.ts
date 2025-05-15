import { reactive } from "vue";
import { AppConfig, Vault } from "./types/VaultType";

interface states {
    active_vault_path: string,

}
const state: states = reactive({
    active_vault_path: ""
})

const changePath = (newPath: string) => {
    state.active_vault_path = newPath;
}
export default {
    state: state.active_vault_path,
    changePath,
}