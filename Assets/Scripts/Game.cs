using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;
using ObjectExtensions;
using Unity.VisualScripting;
using UnityEditor;
using UnityEngine.SceneManagement;

public class Game : MonoBehaviour
{
    [Header( "Data" )]
    [SerializeField] public Scene activeScene;

    [SerializeField] public GameObject stateController;
    [SerializeField] public GameObject player;

    public GameObject worldCanvas;

    public GameObject floatingDamageTextPrefab;

    private GameObject _statScreen;

    [Header( "Broadcast Events" )]
    [SerializeField] public GameObjectEventChannelSO m_ChangeCameraTarget;

    [SerializeField] public ChangeStateEventChannelSO    m_StateChange;
    [SerializeField] public InitiateBattleEventChannelSO m_InitiateBattle;

    [Header( "Listen to Event" )]
    [SerializeField] public InitiateBattleEventChannelSO m_OnInitiateBattle;

    [SerializeField] private VoidEventChannelSO m_OpenStatScreen;

    // Start is called before the first frame update
    private void Start()
    {
        DontDestroyOnLoad( gameObject );

        stateController.GetComponent< StateController >().game = this;

        player = GameObject.FindWithTag( "Player" );
        DontDestroyOnLoad( player.gameObject );

        ConnectEvents();

        m_StateChange.IsValid()?.RaiseEvent( new GameObject().AddComponent< ExplorationState >(), TransitionType.Add );
        m_ChangeCameraTarget.IsValid()?.RaiseEvent( player );

        SceneManager.LoadScene( "Dungeon", LoadSceneMode.Single );

    }

    private void ConnectEvents()
    {
        if ( m_OpenStatScreen.IsValid() ) m_OpenStatScreen.OnEventRaised += OnOpenStatScreen;
    }

    public void OnOpenStatScreen()
    {
        if ( _statScreen.IsValid() )
        {
            _statScreen.SetActive( !_statScreen.activeSelf );
        }
        else
        {
            var s = Resources.Load< GameObject >( "UI/StatusHUD" );
            _statScreen = Instantiate( s );

            _statScreen.GetComponent< StatScreen >().observedEntity = player.GetComponent<Entity>();
        }

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