defmodule Domain.Auth.Adapters.GoogleWorkspace do
  use Supervisor
  alias Domain.Actors
  alias Domain.Auth.{Provider, Adapter}
  alias Domain.Auth.Adapters.OpenIDConnect
  alias Domain.Auth.Adapters.GoogleWorkspace
  require Logger

  @behaviour Adapter
  @behaviour Adapter.IdP

  def start_link(_init_arg) do
    Supervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  @impl true
  def init(_init_arg) do
    children = [
      GoogleWorkspace.APIClient,
      {Domain.Jobs, GoogleWorkspace.Jobs}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end

  @impl true
  def capabilities do
    [
      provisioners: [:just_in_time, :custom],
      default_provisioner: :custom,
      parent_adapter: :openid_connect
    ]
  end

  @impl true
  def identity_changeset(%Provider{} = _provider, %Ecto.Changeset{} = changeset) do
    changeset
    |> Domain.Validator.trim_change(:provider_identifier)
    |> Domain.Validator.copy_change(:provider_virtual_state, :provider_state)
    |> Ecto.Changeset.put_change(:provider_virtual_state, %{})
  end

  @impl true
  def provider_changeset(%Ecto.Changeset{} = changeset) do
    changeset
    |> Domain.Changeset.cast_polymorphic_embed(:adapter_config,
      required: true,
      with: fn current_attrs, attrs ->
        Ecto.embedded_load(GoogleWorkspace.Settings, current_attrs, :json)
        |> GoogleWorkspace.Settings.Changeset.changeset(attrs)
      end
    )
  end

  @impl true
  def ensure_provisioned(%Provider{} = provider) do
    {:ok, provider}
  end

  @impl true
  def ensure_deprovisioned(%Provider{} = provider) do
    {:ok, provider}
  end

  @impl true
  def verify_and_update_identity(%Provider{} = provider, payload) do
    OpenIDConnect.verify_and_update_identity(provider, payload)
  end

  def verify_and_upsert_identity(%Actors.Actor{} = actor, %Provider{} = provider, payload) do
    OpenIDConnect.verify_and_upsert_identity(actor, provider, payload)
  end

  def refresh_access_token(%Provider{} = provider) do
    OpenIDConnect.refresh_access_token(provider)
  end
end
