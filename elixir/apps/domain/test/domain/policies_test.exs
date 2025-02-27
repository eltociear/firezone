defmodule Domain.PoliciesTest do
  alias Web.Policies
  use Domain.DataCase, async: true
  import Domain.Policies
  alias Domain.Policies

  setup do
    account = Fixtures.Accounts.create_account()
    actor = Fixtures.Actors.create_actor(type: :account_admin_user, account: account)
    identity = Fixtures.Auth.create_identity(account: account, actor: actor)
    subject = Fixtures.Auth.create_subject(identity: identity)

    %{
      account: account,
      actor: actor,
      identity: identity,
      subject: subject
    }
  end

  describe "fetch_policy_by_id/2" do
    test "returns error when policy does not exist", %{subject: subject} do
      assert fetch_policy_by_id(Ecto.UUID.generate(), subject) == {:error, :not_found}
    end

    test "returns error when UUID is invalid", %{subject: subject} do
      assert fetch_policy_by_id("foo", subject) == {:error, :not_found}
    end

    test "returns policy when policy exists", %{account: account, subject: subject} do
      policy = Fixtures.Policies.create_policy(account: account)

      assert {:ok, fetched_policy} = fetch_policy_by_id(policy.id, subject)
      assert fetched_policy.id == policy.id
    end

    test "does not return deleted policy", %{account: account, subject: subject} do
      {:ok, policy} =
        Fixtures.Policies.create_policy(account: account)
        |> delete_policy(subject)

      assert fetch_policy_by_id(policy.id, subject) == {:error, :not_found}
    end

    test "does not return policies in other accounts", %{subject: subject} do
      policy = Fixtures.Policies.create_policy()
      assert fetch_policy_by_id(policy.id, subject) == {:error, :not_found}
    end

    test "returns error when subject has no permission to view policies", %{subject: subject} do
      subject = Fixtures.Auth.remove_permissions(subject)

      assert fetch_policy_by_id(Ecto.UUID.generate(), subject) ==
               {:error,
                {:unauthorized,
                 [
                   missing_permissions: [
                     {:one_of,
                      [
                        Policies.Authorizer.manage_policies_permission(),
                        Policies.Authorizer.view_available_policies_permission()
                      ]}
                   ]
                 ]}}
    end

    # TODO: add a test that soft-deleted assocs are not preloaded
    test "associations are preloaded when opts given", %{account: account, subject: subject} do
      policy = Fixtures.Policies.create_policy(account: account)
      {:ok, policy} = fetch_policy_by_id(policy.id, subject, preload: [:actor_group, :resource])

      assert Ecto.assoc_loaded?(policy.actor_group)
      assert Ecto.assoc_loaded?(policy.resource)
    end
  end

  describe "list_policies/1" do
    test "returns empty list when there are no policies", %{subject: subject} do
      assert list_policies(subject) == {:ok, []}
    end

    test "does not list policies from other accounts", %{subject: subject} do
      Fixtures.Policies.create_policy()
      assert list_policies(subject) == {:ok, []}
    end

    test "does not list deleted policies", %{account: account, subject: subject} do
      Fixtures.Policies.create_policy(account: account)
      |> delete_policy(subject)

      assert list_policies(subject) == {:ok, []}
    end

    test "returns all policies for account admin subject", %{account: account} do
      actor = Fixtures.Actors.create_actor(type: :account_admin_user, account: account)
      identity = Fixtures.Auth.create_identity(account: account, actor: actor)
      subject = Fixtures.Auth.create_subject(identity: identity)

      Fixtures.Policies.create_policy(account: account)
      Fixtures.Policies.create_policy(account: account)
      Fixtures.Policies.create_policy()

      assert {:ok, policies} = list_policies(subject)
      assert length(policies) == 2
    end

    test "returns select policies for non-admin subject", %{account: account, subject: subject} do
      unprivileged_actor = Fixtures.Actors.create_actor(type: :account_user, account: account)

      unprivileged_subject =
        Fixtures.Auth.create_subject(account: account, identity: [actor: unprivileged_actor])

      actor_group = Fixtures.Actors.create_group(account: account, subject: subject)

      Fixtures.Actors.create_membership(
        account: account,
        actor: unprivileged_actor,
        group: actor_group
      )

      Fixtures.Policies.create_policy(account: account, actor_group: actor_group)
      Fixtures.Policies.create_policy(account: account)
      Fixtures.Policies.create_policy()

      assert {:ok, policies} = list_policies(unprivileged_subject)
      assert length(policies) == 1
    end

    test "returns error when subject has no permission to view policies", %{subject: subject} do
      subject = Fixtures.Auth.remove_permissions(subject)

      assert list_policies(subject) ==
               {:error,
                {:unauthorized,
                 [
                   missing_permissions: [
                     {:one_of,
                      [
                        Policies.Authorizer.manage_policies_permission(),
                        Policies.Authorizer.view_available_policies_permission()
                      ]}
                   ]
                 ]}}
    end
  end

  describe "create_policy/2" do
    test "returns changeset error on empty params", %{subject: subject} do
      assert {:error, changeset} = create_policy(%{}, subject)

      assert errors_on(changeset) == %{
               name: ["can't be blank"],
               actor_group_id: ["can't be blank"],
               resource_id: ["can't be blank"]
             }
    end

    test "returns changeset error on invalid params", %{subject: subject} do
      assert {:error, changeset} =
               create_policy(
                 %{name: 1, actor_group_id: "foo", resource_id: "bar"},
                 subject
               )

      assert errors_on(changeset) == %{name: ["is invalid"]}
    end

    test "returns error when subject has no permission to manage policies", %{subject: subject} do
      subject = Fixtures.Auth.remove_permissions(subject)

      assert create_policy(%{}, subject) ==
               {:error,
                {:unauthorized,
                 [
                   missing_permissions: [
                     {:one_of, [Policies.Authorizer.manage_policies_permission()]}
                   ]
                 ]}}
    end

    test "returns changeset error when trying to create policy with another account actor_group",
         %{
           account: account,
           subject: subject
         } do
      other_account = Fixtures.Accounts.create_account()

      resource = Fixtures.Resources.create_resource(account: account)
      other_actor_group = Fixtures.Actors.create_group(account: other_account)

      attrs = %{
        account_id: account.id,
        name: "yikes",
        actor_group_id: other_actor_group.id,
        resource_id: resource.id
      }

      assert {:error, changeset} = create_policy(attrs, subject)

      assert errors_on(changeset) == %{actor_group: ["does not exist"]}
    end

    test "returns changeset error when trying to create policy with another account resource", %{
      account: account,
      subject: subject
    } do
      other_account = Fixtures.Accounts.create_account()

      other_resource = Fixtures.Resources.create_resource(account: other_account)
      actor_group = Fixtures.Actors.create_group(account: account)

      attrs = %{
        account_id: account.id,
        name: "yikes",
        actor_group_id: actor_group.id,
        resource_id: other_resource.id
      }

      assert {:error, changeset} =
               create_policy(attrs, subject)

      assert errors_on(changeset) == %{resource: ["does not exist"]}
    end
  end

  describe "update_policy/3" do
    setup context do
      policy =
        Fixtures.Policies.create_policy(
          account: context.account,
          subject: context.subject
        )

      Map.put(context, :policy, policy)
    end

    test "does nothing on empty params", %{policy: policy, subject: subject} do
      assert {:ok, _policy} = update_policy(policy, %{}, subject)
    end

    test "returns changeset error on invalid params", %{account: account, subject: subject} do
      policy = Fixtures.Policies.create_policy(account: account, subject: subject)

      assert {:error, changeset} =
               update_policy(
                 policy,
                 %{name: 1, actor_group_id: "foo", resource_id: "bar"},
                 subject
               )

      assert errors_on(changeset) == %{name: ["is invalid"]}
    end

    test "allows update to name", %{policy: policy, subject: subject} do
      assert {:ok, updated_policy} =
               update_policy(policy, %{name: "updated policy name"}, subject)

      assert updated_policy.name == "updated policy name"
    end

    test "does not allow update to actor_group_id", %{
      policy: policy,
      account: account,
      subject: subject
    } do
      new_actor_group = Fixtures.Actors.create_group(account: account)

      assert {:ok, updated_policy} =
               update_policy(policy, %{actor_group_id: new_actor_group.id}, subject)

      assert updated_policy.actor_group_id == policy.actor_group_id
    end

    test "does not allow update to resource_id", %{
      policy: policy,
      account: account,
      subject: subject
    } do
      new_resource = Fixtures.Resources.create_resource(account: account)

      assert {:ok, updated_policy} =
               update_policy(policy, %{resource_id: new_resource.id}, subject)

      assert updated_policy.resource_id == policy.resource_id
    end

    test "returns error when subject has no permission to update policies", %{
      policy: policy,
      subject: subject
    } do
      subject = Fixtures.Auth.remove_permissions(subject)

      assert update_policy(policy, %{name: "Name Change Attempt"}, subject) ==
               {:error,
                {:unauthorized,
                 [
                   missing_permissions: [
                     {:one_of, [Policies.Authorizer.manage_policies_permission()]}
                   ]
                 ]}}
    end

    test "return error when subject is outside of account", %{policy: policy} do
      other_account = Fixtures.Accounts.create_account()

      other_actor =
        Fixtures.Actors.create_actor(type: :account_admin_user, account: other_account)

      other_identity = Fixtures.Auth.create_identity(account: other_account, actor: other_actor)
      other_subject = Fixtures.Auth.create_subject(identity: other_identity)

      assert {:error, :unauthorized} =
               update_policy(policy, %{name: "Should not be allowed"}, other_subject)
    end
  end

  describe "delete_policy/2" do
    setup context do
      policy =
        Fixtures.Policies.create_policy(
          account: context.account,
          subject: context.subject
        )

      Map.put(context, :policy, policy)
    end

    test "deletes policy", %{policy: policy, subject: subject} do
      assert {:ok, deleted_policy} = delete_policy(policy, subject)
      assert deleted_policy.deleted_at != nil
    end

    test "returns error when subject has no permission to delete policies", %{
      policy: policy,
      subject: subject
    } do
      subject = Fixtures.Auth.remove_permissions(subject)

      assert delete_policy(policy, subject) ==
               {:error,
                {:unauthorized,
                 [
                   missing_permissions: [
                     {:one_of, [Policies.Authorizer.manage_policies_permission()]}
                   ]
                 ]}}
    end

    test "returns error on state conflict", %{policy: policy, subject: subject} do
      assert {:ok, deleted_policy} = delete_policy(policy, subject)
      assert delete_policy(deleted_policy, subject) == {:error, :not_found}
      assert delete_policy(policy, subject) == {:error, :not_found}
    end

    test "returns error when subject attempts to delete policy outside of account", %{
      policy: policy
    } do
      other_subject = Fixtures.Auth.create_subject()
      assert delete_policy(policy, other_subject) == {:error, :not_found}
    end
  end
end
