defmodule Domain.Fixtures.Config do
  use Domain.Fixture
  alias Domain.Config

  def configuration_attrs(attrs \\ %{}) do
    Enum.into(attrs, %{
      devices_upstream_dns: ["1.1.1.1"]
    })
  end

  def upsert_configuration(attrs \\ %{}) do
    attrs = configuration_attrs(attrs)

    {account, attrs} =
      pop_assoc_fixture(attrs, :account, fn assoc_attrs ->
        Fixtures.Accounts.create_account(assoc_attrs)
      end)

    {:ok, configuration} =
      Config.get_account_config_by_account_id(account.id)
      |> Config.update_config(attrs)

    configuration
  end

  def set_config(account, key, value) do
    upsert_configuration([{:account, account}, {key, value}])
  end
end
