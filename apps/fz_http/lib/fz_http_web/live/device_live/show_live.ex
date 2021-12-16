defmodule FzHttpWeb.DeviceLive.Show do
  @moduledoc """
  Handles Device LiveViews.
  """
  use FzHttpWeb, :live_view

  alias FzHttp.{Devices, Users}

  @impl Phoenix.LiveView
  def mount(params, session, socket) do
    {:ok,
     socket
     |> assign(:dropdown_active_class, "")
     |> assign_defaults(params, session, &load_data/2)}
  end

  @impl Phoenix.LiveView
  def handle_params(_params, _url, socket) do
    {:noreply, socket}
  end

  @impl Phoenix.LiveView
  def handle_event("create_config_token", _params, socket) do
    device = socket.assigns.device

    if device.user_id == socket.assigns.current_user.id do
      case Devices.create_config_token(device) do
        {:ok, device} ->
          {:noreply,
           socket
           |> assign(:dropdown_active_class, "is-active")
           |> assign(:device, device)}

        {:error, _changeset} ->
          {:noreply,
           socket
           |> put_flash(:error, "Could not create device config token.")}
      end
    else
      {:noreply, not_authorized(socket)}
    end
  end

  @impl Phoenix.LiveView
  def handle_event("hide_config_token", _params, socket) do
    {:noreply,
     socket
     |> assign(:dropdown_active_class, "")}
  end

  @impl Phoenix.LiveView
  def handle_event("delete_device", _params, socket) do
    device = socket.assigns.device

    if device.user_id == socket.assigns.current_user.id do
      case Devices.delete_device(device) do
        {:ok, _deleted_device} ->
          {:ok, _deleted_pubkey} = @events_module.delete_device(device.public_key)

          {:noreply,
           socket
           |> redirect(to: Routes.device_index_path(socket, :index))}

          # Not likely to ever happen
          # {:error, msg} ->
          #   {:noreply,
          #   socket
          #   |> put_flash(:error, "Error deleting device: #{msg}")}
      end
    else
      {:noreply, not_authorized(socket)}
    end
  end

  defp load_data(%{"id" => id}, socket) do
    device = Devices.get_device!(id)

    if device.user_id == socket.assigns.current_user.id do
      socket
      |> assign(
        device: device,
        user: Users.get_user!(device.user_id),
        page_title: device.name,
        allowed_ips: Devices.allowed_ips(device),
        dns_servers: Devices.dns_servers(device),
        endpoint: Devices.endpoint(device),
        config: Devices.as_config(device)
      )
    else
      not_authorized(socket)
    end
  end
end
