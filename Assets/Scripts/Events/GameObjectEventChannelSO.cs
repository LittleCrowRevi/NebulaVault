using Unity;
using UnityEngine;
using UnityEngine.Events;

[CreateAssetMenu(menuName = "Events/Game Object Event Channel")]
public class GameObjectEventChannelSO : ScriptableObject
{
    public UnityAction< GameObject > OnRaiseEvent;

    public void RaiseEvent( GameObject gameObject )
    {
        OnRaiseEvent?.Invoke( gameObject );
    }
}