using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;
using ObjectExtensions;
using UnityEditor;
using UnityEngine.SceneManagement;

public class Game : MonoBehaviour
{
    [Header( "Data" )]
    [SerializeField] public Scene activeScene;

    [SerializeField] public GameObject stateController;
    [SerializeField] public GameObject player;

    [Header( "Broadcast Events" )]
    [SerializeField] public GameObjectEventChannelSO m_ChangeCameraTarget;

    [SerializeField] public ChangeStateEventChannelSO    m_StateChange;
    [SerializeField] public InitiateBattleEventChannelSO m_InitiateBattle;

    [Header( "Listen to Event" )]
    [SerializeField] public InitiateBattleEventChannelSO m_OnInitiateBattle;

    // Start is called before the first frame update
    private void Start()
    {
        DontDestroyOnLoad( gameObject );

        stateController.GetComponent< StateController >().Game = this;

        player = GameObject.FindWithTag( "Player" );
        DontDestroyOnLoad( player.gameObject );

        m_StateChange.IsValid()?.RaiseEvent( new ExplorationState(), TransitionType.Add );
        m_ChangeCameraTarget.IsValid()?.RaiseEvent( player );

        SceneManager.LoadScene( "Dungeon", LoadSceneMode.Single );

    }
}

namespace ObjectExtensions
{
    public static class Extensions
    {
        public static T IsValid< T >( this T unityObject ) where T : UnityEngine.Object
        {
            return !unityObject ? null : unityObject;
        }
    }
}


public class ConditionalPropertyAttribute : PropertyAttribute
{
    public string   condition;
    public object[] compareValues;

    public ConditionalPropertyAttribute( string condition )
    {
        this.condition = condition;
    }

    public ConditionalPropertyAttribute( string fieldToCheck, params object[] values )
    {
        this.condition     = fieldToCheck;
        this.compareValues = values;
    }
}