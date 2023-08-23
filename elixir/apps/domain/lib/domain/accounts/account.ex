defmodule Domain.Accounts.Account do
  use Domain, :schema

  @derive {Phoenix.Param, key: :slug}

  schema "accounts" do
    field :name, :string
    field :slug, :string

    timestamps()
  end
end
