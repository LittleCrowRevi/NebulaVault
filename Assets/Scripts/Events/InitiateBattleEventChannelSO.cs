using UnityEngine;
using UnityEngine.Events;

[CreateAssetMenu(menuName = "Events/Initiate Battle Event Channel")]
public class InitiateBattleEventChannelSO : ScriptableObject
{

    public delegate void Event( GameObject[] friendlyActors, GameObject[] hostileActors );

    public event Event OnRaiseEvent;

    public void RaiseEvent( GameObject[] friendlyActors, GameObject[] hostileActors )
    {
        OnRaiseEvent?.Invoke( friendlyActors, hostileActors );
    }
}