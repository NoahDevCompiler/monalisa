export type Vault = {
  name: string;
  path: string;
};

export type AppConfig = {
  vaults: Vault[];
  active_vault: string | null;
};