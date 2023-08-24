defmodule Web.TableComponents do
  @moduledoc """
  Provides Table UI components.
  """
  use Phoenix.Component
  use Web, :verified_routes
  import Web.Gettext
  import Web.CoreComponents

  attr :columns, :any, required: true, doc: "col slot taken from parent component"
  attr :actions, :any, required: true, doc: "action slot taken from parent component"

  def table_header(assigns) do
    ~H"""
    <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th :for={col <- @columns} class="px-4 py-3">
          <%= col[:label] %>
          <.icon
            :if={col[:sortable] == "true"}
            name="hero-chevron-up-down-solid"
            class="w-4 h-4 ml-1"
          />
        </th>
        <th :if={not Enum.empty?(@actions)} class="px-4 py-3">
          <span class="sr-only"><%= gettext("Actions") %></span>
        </th>
      </tr>
    </thead>
    """
  end

  attr :id, :any, default: nil, doc: "the function for generating the row id"
  attr :row, :map, required: true, doc: "the row data"
  attr :click, :any, default: nil, doc: "the function for handling phx-click on each row"

  attr :columns, :any, required: true, doc: "col slot taken from parent component"
  attr :actions, :list, required: true, doc: "action slot taken from parent component"

  attr :mapper, :any,
    default: &Function.identity/1,
    doc: "the function for mapping each row before calling the :col and :action slots"

  def table_row(assigns) do
    ~H"""
    <tr id={@id} class="border-b dark:border-gray-700">
      <td
        :for={{col, _i} <- Enum.with_index(@columns)}
        phx-click={@click && @click.(@row)}
        class={[
          "px-4 py-3",
          @click && "hover:cursor-pointer"
        ]}
      >
        <%= render_slot(col, @mapper.(@row)) %>
      </td>
      <td :if={@actions != []} class="px-4 py-3 flex items-center justify-end">
        <button id={"#{@id}-dropdown-button"} data-dropdown-toggle={"#{@id}-dropdown"} class={~w[
                  inline-flex items-center p-0.5 text-sm font-medium text-center
                  text-gray-500 hover:text-gray-800 rounded-lg focus:outline-none
                  dark:text-gray-400 dark:hover:text-gray-100
                ]} type="button">
          <.icon name="hero-ellipsis-horizontal" class="w-5 h-5" />
        </button>
        <div id={"#{@id}-dropdown" } class={~w[
                  hidden z-10 w-44 bg-white rounded divide-y divide-gray-100
                  shadow border border-gray-300 dark:bg-gray-700 dark:divide-gray-600"
                ]}>
          <ul
            class="py-1 text-sm text-gray-700 dark:text-gray-200"
            aria-labelledby={"#{@id}-dropdown-button"}
          >
            <li :for={action <- @actions}>
              <%= render_slot(action, @mapper.(@row)) %>
            </li>
          </ul>
        </div>
      </td>
    </tr>
    """
  end

  @doc ~S"""
  Renders a table with generic styling.

  ## Examples

      <.table id="users" rows={@users}>
        <:col :let={user} label="id"><%= user.id %></:col>
        <:col :let={user} label="username"><%= user.username %></:col>
      </.table>
  """
  attr :id, :string, required: true
  attr :rows, :list, required: true
  attr :row_id, :any, default: nil, doc: "the function for generating the row id"
  attr :row_click, :any, default: nil, doc: "the function for handling phx-click on each row"

  attr :row_item, :any,
    default: &Function.identity/1,
    doc: "the function for mapping each row before calling the :col and :action slots"

  slot :col, required: true do
    attr :label, :string
    attr :sortable, :string
  end

  slot :action, doc: "the slot for showing user actions in the last table column"

  def table(assigns) do
    assigns =
      with %{rows: %Phoenix.LiveView.LiveStream{}} <- assigns do
        assign(assigns, row_id: assigns.row_id || fn {id, _item} -> id end)
      end

    ~H"""
    <div class="overflow-x-auto">
      <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
        <.table_header columns={@col} actions={@action} />
        <tbody id={@id} phx-update={match?(%Phoenix.LiveView.LiveStream{}, @rows) && "stream"}>
          <.table_row
            :for={row <- @rows}
            columns={@col}
            actions={@action}
            row={row}
            id={@row_id && @row_id.(row)}
            click={@row_click}
            mapper={@row_item}
          />
        </tbody>
      </table>
    </div>
    """
  end

  @doc ~S"""
  Renders a table with groups and generic styling.

  The component is expecting the rows data to be in the form of a list
  of tuples, where the first element of a given tuple is the group and
  the second element of the tuple is a list of elements under that group

  ## Examples

      <.table_with_groups id="users" rows={@grouped_users}>
        <:col label="user group"></:col>
        <:col :let={user} label="id"><%= user.id %></:col>
        <:col :let={user} label="username"><%= user.username %></:col>
      </.table>
  """

  attr :id, :string, required: true
  attr :groups, :list, required: true
  attr :group_id, :any, default: nil, doc: "the function for generating the group id"

  attr :row_id, :any, default: nil, doc: "the function for generating the row id"
  attr :row_click, :any, default: nil, doc: "the function for handling phx-click on each row"

  attr :group_items, :any,
    required: true,
    doc: "a mapper which is used to get list of rows for a group"

  attr :row_item, :any,
    default: &Function.identity/1,
    doc: "the function for mapping each row before calling the :col and :action slots"

  slot :col, required: true do
    attr :label, :string
    attr :sortable, :string
  end

  slot :group, required: true

  slot :action, doc: "the slot for showing user actions in the last table column"

  def table_with_groups(assigns) do
    assigns =
      with %{rows: %Phoenix.LiveView.LiveStream{}} <- assigns do
        assign(assigns, row_id: assigns.row_id || fn {id, _item} -> id end)
      end

    ~H"""
    <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
      <.table_header columns={@col} actions={@action} />

      <tbody :for={group <- @groups} data-group-id={@group_id && @group_id.(group)}>
        <tr class="bg-gray-100">
          <td class="px-4 py-2" colspan={length(@col) + 1}>
            <%= render_slot(@group, group) %>
          </td>
        </tr>

        <.table_row
          :for={row <- @group_items.(group)}
          columns={@col}
          actions={@action}
          row={row}
          id={@row_id && @row_id.(row)}
          click={@row_click}
          mapper={@row_item}
        />
      </tbody>
    </table>
    """
  end

  @doc ~S"""
  Renders a table with 2 columns and generic styling.

  The component will likely be used when displaying the properties of an
  individual resource (e.g. Gateway, Resource, Device, etc...)

  The component renders a table that is meant to be viewed vertically, with
  the first column being the label and the second column being the value.

  This component is intended to be used with the `vertical_table_row` component

  ## Examples

      <.vertical_table>
        <.vertical_table_row>
          <:label>First Name</:label>
          <:value>User First Name Here</:value>
        </.vertical_table_row>
        <.vertical_table_row>
          <:label>Last Name</:label>
          <:value>User Last Name Here</:value>
        </.vertical_table_row>
      </.vertical_table>
  """

  attr :class, :string, default: nil
  attr :rest, :global

  slot :inner_block

  def vertical_table(assigns) do
    ~H"""
    <table class={["w-full text-sm text-left text-gray-500 dark:text-gray-400", @class]}>
      <tbody>
        <%= render_slot(@inner_block) %>
      </tbody>
    </table>
    """
  end

  @doc ~S"""
  Renders a row with 2 columns and generic styling.  The first column will be
  the header and the second column will be the value.

  The component will likely be used when displaying the properties of an
  individual resource (e.g. Gateway, Resource, Device, etc...)

  This component is intended to be used with the `vertical_table` component.

  ## Examples

      <.vertical_table_row>
        <:label>First Name</:label>
        <:value>User First Name Here</:value>
      </.vertical_table_row>
  """

  attr :label_class, :string, default: nil
  attr :value_class, :string, default: nil

  slot :label, doc: "the slot for rendering the label of a row"
  slot :value, doc: "the slot for rendering the value of a row"

  def vertical_table_row(assigns) do
    ~H"""
    <tr class="border-b border-gray-200 dark:border-gray-700">
      <th
        scope="row"
        class={[
          "text-right px-6 py-4 font-medium text-gray-900 whitespace-nowrap",
          "bg-gray-50 dark:text-white dark:bg-gray-800",
          @label_class
        ]}
      >
        <%= render_slot(@label) %>
      </th>
      <td class={["px-6 py-4", @value_class]}>
        <%= render_slot(@value) %>
      </td>
    </tr>
    """
  end

  @doc ~S"""
  This component is meant to be used with the table component.  It renders a
  <.link> component that has a specific style for actions in a table.
  """
  attr :navigate, :string, required: true
  slot :inner_block

  def action_link(assigns) do
    ~H"""
    <.link
      navigate={@navigate}
      class="block py-2 px-4 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
    >
      <%= render_slot(@inner_block) %>
    </.link>
    """
  end
end
